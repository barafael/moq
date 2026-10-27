//! One immutable content owner, independent of each transport delivery's progress.
use super::*;

/// Wire Object IDs are not frame ordinals on a sparse IETF stream.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(super) enum Key {
	Frame(u64),
	Object(u64),
}

pub(super) enum Input {
	Whole(Payload),
	Stream(Option<FrameBuf>),
	Adopt(Arc<Row>),
}

pub(super) struct Spec {
	pub key: Key,
	pub size: u64,
	pub inflated: Option<u64>,
}

pub(super) struct Row {
	key: Key,
	inflated: Option<u64>,
	pub(super) buf: FrameBuf,
	bill: cache::Extra,
}

pub(in crate::model) struct Content {
	state: Mutex<Stored>,
	bill: cache::Extra,
	plain: Mutex<Option<Arc<Plain>>>,
	views: AtomicUsize,
	index: Mutex<Option<(Arc<Index>, u64)>>,
}

#[derive(Default)]
struct Stored {
	rows: Vec<Arc<Row>>,
	declared: u64,
	inflated: u64,
}

impl Content {
	pub(super) fn new(charge: &cache::Charge, plain: Option<Arc<Plain>>) -> Arc<Self> {
		let bill = charge.extra();
		bill.add((2 * size_of::<usize>() + size_of::<Self>()) as u64);
		Arc::new(Self {
			state: Mutex::default(),
			bill,
			plain: Mutex::new(plain),
			views: AtomicUsize::new(0),
			index: Mutex::default(),
		})
	}

	pub(super) fn key(index: usize, fields: Option<&super::super::object::Fields>, wire: bool) -> Key {
		if wire {
			Key::Object(fields.map_or(index as u64, |fields| fields.id))
		} else {
			Key::Frame(index as u64)
		}
	}

	pub(super) fn row(
		&self,
		spec: Spec,
		input: Input,
		charge: &cache::Charge,
		transferred: Option<cache::Extra>,
	) -> Result<Arc<Row>> {
		let Spec { key, size, inflated } = spec;
		let mut state = self.state.lock().unwrap_or_else(PoisonError::into_inner);
		let at = match state.rows.binary_search_by_key(&key, |row| row.key) {
			Ok(at) => {
				let row = &state.rows[at];
				if row.buf.size() as u64 != size || row.inflated != inflated {
					return Err(Error::ProtocolViolation);
				}
				let payload = match input {
					Input::Whole(payload) => Some(payload),
					Input::Adopt(row) => Some(row.buf.freeze()),
					Input::Stream(_) => None,
				};
				if let Some(payload) = payload {
					let delivery = row.buf.fork();
					let mut at = 0;
					while at < payload.len() {
						let chunk = payload.chunk_at(at);
						delivery.append(&chunk)?;
						at += chunk.len();
					}
				}
				return Ok(row.clone());
			}
			Err(at) => at,
		};
		if size > MAX_CACHE_BYTES || inflated.is_some_and(|size| size > MAX_CACHE_BYTES) {
			return Err(Error::FrameTooLarge);
		}
		if state.rows.len() == MAX_GROUP_FRAMES
			|| state.declared.saturating_add(size) > MAX_CACHE_BYTES
			|| state.inflated.saturating_add(inflated.unwrap_or(0)) > MAX_CACHE_BYTES
		{
			return Err(Error::GroupTooLarge);
		}
		let row = match input {
			Input::Adopt(row) if row.key == key && row.inflated == inflated => row,
			input => {
				let buf = match input {
					Input::Whole(payload) => FrameBuf::from_payload(payload),
					Input::Stream(Some(buf)) => buf,
					Input::Stream(None) => FrameBuf::new(size as usize),
					Input::Adopt(row) => FrameBuf::from_payload(row.buf.freeze()),
				};
				buf.bill_content(transferred.unwrap_or_else(|| charge.extra()));
				buf.bill_view(charge.extra());
				let bill = charge.extra();
				bill.add((2 * size_of::<usize>() + size_of::<Row>()) as u64);
				Arc::new(Row {
					key,
					inflated,
					buf,
					bill,
				})
			}
		};
		let before = state.rows.capacity();
		state.rows.insert(at, row.clone());
		self.bill
			.add(((state.rows.capacity() - before) * size_of::<Arc<Row>>()) as u64);
		state.declared += size;
		state.inflated += inflated.unwrap_or(0);
		Ok(row)
	}

	pub(super) fn attach(&self, charge: &cache::Charge) -> Option<Arc<Plain>> {
		let mut plain = self.plain.lock().unwrap_or_else(PoisonError::into_inner);
		self.views.fetch_add(1, Ordering::Relaxed);
		if plain.as_ref().is_some_and(|plain| plain.lock().released.is_some()) {
			*plain = Some(Arc::new(Plain::new(charge.extra())));
		}
		plain.clone()
	}

	pub(super) fn detach(&self, err: Error) {
		let plain = self.plain.lock().unwrap_or_else(PoisonError::into_inner);
		if self.views.fetch_sub(1, Ordering::AcqRel) == 1
			&& let Some(plain) = plain.as_ref()
		{
			plain.release(err);
		}
	}

	#[cfg(test)]
	pub(in crate::model) fn cost(&self) -> (usize, usize, u64) {
		let state = self.state.lock().unwrap_or_else(PoisonError::into_inner);
		let rows = state.rows.len();
		let bytes = state
			.rows
			.iter()
			.map(|row| row.buf.shared_written(Ordering::Acquire))
			.sum();
		drop(state);
		(rows, bytes, self.size())
	}

	pub(super) fn size(&self) -> u64 {
		self.bill.size()
			+ self
				.plain
				.lock()
				.unwrap_or_else(PoisonError::into_inner)
				.as_ref()
				.map_or(0, |plain| plain.bill.size())
			+ self
				.state
				.lock()
				.unwrap_or_else(PoisonError::into_inner)
				.rows
				.iter()
				.map(|row| row.bill.size() + row.buf.content_size() + row.buf.view_size())
				.sum::<u64>()
	}
}

impl Drop for Content {
	fn drop(&mut self) {
		if let Some(plain) = self.plain.get_mut().unwrap_or_else(PoisonError::into_inner)
			&& plain.lock().released.is_none()
		{
			plain.release(Error::Dropped);
		}
		if let Some((index, sequence)) = self.index.get_mut().unwrap_or_else(PoisonError::into_inner).take() {
			let mut state = index.state.lock().unwrap_or_else(PoisonError::into_inner);
			if let Ok(at) = state.binary_search_by_key(&sequence, |(sequence, _)| *sequence)
				&& std::ptr::eq(state[at].1.as_ptr(), self)
			{
				state.remove(at);
			}
		}
	}
}

/// Only live content owners occupy the index; a dead owner removes its weak entry.
/// A new publisher incarnation replaces the entry instead of inheriting old bytes.
#[derive(Default)]
pub(in crate::model) struct Contents(Mutex<Weak<Index>>);

#[derive(Default)]
struct Index {
	state: Mutex<Vec<(u64, Weak<Content>)>>,
	bill: OnceLock<cache::Extra>,
}

impl Contents {
	pub(in crate::model) fn get(&self, sequence: u64) -> Option<Arc<Content>> {
		let index = self.0.lock().unwrap_or_else(PoisonError::into_inner).upgrade()?;
		let state = index.state.lock().unwrap_or_else(PoisonError::into_inner);
		state
			.binary_search_by_key(&sequence, |(sequence, _)| *sequence)
			.ok()
			.and_then(|at| state[at].1.upgrade())
	}

	pub(in crate::model) fn remove(&self, sequence: u64) {
		let Some(index) = self.0.lock().unwrap_or_else(PoisonError::into_inner).upgrade() else {
			return;
		};
		let mut state = index.state.lock().unwrap_or_else(PoisonError::into_inner);
		if let Ok(at) = state.binary_search_by_key(&sequence, |(sequence, _)| *sequence) {
			state.remove(at);
		}
	}

	pub(in crate::model) fn insert(&self, sequence: u64, content: &Arc<Content>, charge: &cache::Charge) {
		let index = {
			let mut current = self.0.lock().unwrap_or_else(PoisonError::into_inner);
			match current.upgrade() {
				Some(index) => index,
				None => {
					let index = Arc::new(Index::default());
					*current = Arc::downgrade(&index);
					index
				}
			}
		};
		let mut state = index.state.lock().unwrap_or_else(PoisonError::into_inner);
		let bill = index.bill.get_or_init(|| {
			let bill = charge.extra();
			bill.add((2 * size_of::<usize>() + size_of::<Index>()) as u64);
			bill
		});
		match state.binary_search_by_key(&sequence, |(sequence, _)| *sequence) {
			Ok(at) => state[at].1 = Arc::downgrade(content),
			Err(at) => {
				let before = state.capacity();
				state.insert(at, (sequence, Arc::downgrade(content)));
				bill.add(((state.capacity() - before) * size_of::<(u64, Weak<Content>)>()) as u64);
			}
		}
		drop(state);
		*content.index.lock().unwrap_or_else(PoisonError::into_inner) = Some((index, sequence));
	}
}
