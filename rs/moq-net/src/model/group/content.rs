//! Weak indexed identity; delivery records and streaming readers own the bytes.
use super::*;

/// Object IDs are not frame ordinals on a sparse IETF stream.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(in crate::model) enum Key {
	Frame(u64),
	Object(u64),
}

pub(super) enum Input {
	Whole(Payload),
	Stream(Option<FrameBuf>),
	Adopt(frame::Shared),
}

pub(super) struct Spec {
	pub key: Key,
	pub size: u64,
	pub inflated: Option<u64>,
}

struct Row {
	key: Key,
	inflated: Option<u64>,
	content: frame::SharedWeak,
}

pub(in crate::model) struct Content {
	state: Mutex<Vec<Row>>,
	bill: cache::Extra,
	plain: Mutex<Option<Arc<Plain>>>,
	views: AtomicUsize,
	index: Mutex<Option<(Arc<Index>, u64)>>,
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
		if wire || fields.is_some() {
			Key::Object(fields.map_or(index as u64, |fields| fields.id))
		} else {
			Key::Frame(index as u64)
		}
	}

	pub(super) fn row(
		self: &Arc<Self>,
		spec: Spec,
		input: Input,
		charge: &cache::Charge,
		transferred: Option<cache::Extra>,
	) -> Result<frame::Shared> {
		let Spec { key, size, inflated } = spec;
		let mut state = self.state.lock().unwrap_or_else(PoisonError::into_inner);
		let at = match state.binary_search_by_key(&key, |row| row.key) {
			Ok(at) => {
				if let Some(shared) = state[at].content.upgrade() {
					let matches = shared.size() as u64 == size && state[at].inflated == inflated;
					// A temporary strong owner must never drop under this mutex: its last
					// release removes its weak record through `forget_frame`.
					drop(state);
					if !matches {
						return Err(Error::ProtocolViolation);
					}
					let payload = match input {
						Input::Whole(payload) => Some(payload),
						Input::Adopt(shared) => Some(shared.freeze()),
						Input::Stream(_) => None,
					};
					if let Some(payload) = payload {
						let delivery = shared.fork();
						let mut at = 0;
						while at < payload.len() {
							let chunk = payload.chunk_at(at);
							delivery.append(&chunk)?;
							at += chunk.len();
						}
					}
					return Ok(shared);
				}
				state.remove(at);
				at
			}
			Err(at) => at,
		};
		// Retained bytes/frame limits remain per delivery. Combining valid disjoint
		// ranges must not introduce a stricter aggregate admission policy.
		if size > MAX_CACHE_BYTES || inflated.is_some_and(|size| size > MAX_CACHE_BYTES) {
			return Err(Error::FrameTooLarge);
		}
		let shared = match input {
			Input::Whole(payload) => frame::Shared::from_payload(payload),
			Input::Stream(Some(buf)) => buf.shared(),
			Input::Stream(None) => FrameBuf::new(size as usize).shared(),
			Input::Adopt(shared) => frame::Shared::from_payload(shared.freeze()),
		};
		if !shared.identify(self.clone(), key) {
			drop(state);
			return Err(Error::ProtocolViolation);
		}
		shared.bill_content(transferred.unwrap_or_else(|| charge.extra()));
		let before = state.capacity();
		state.insert(
			at,
			Row {
				key,
				inflated,
				content: shared.downgrade(),
			},
		);
		self.bill.add(((state.capacity() - before) * size_of::<Row>()) as u64);
		drop(state);
		Ok(shared)
	}

	pub(in crate::model) fn forget_frame(&self, key: Key, address: usize) {
		let mut state = self.state.lock().unwrap_or_else(PoisonError::into_inner);
		if let Ok(at) = state.binary_search_by_key(&key, |row| row.key)
			&& state[at].content.address() == address
		{
			state.remove(at);
		}
		if state.is_empty() {
			let bytes = state.capacity() * size_of::<Row>();
			*state = Vec::new();
			self.bill.sub(bytes as u64);
		}
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

	fn frames(&self) -> Vec<frame::Shared> {
		self.state
			.lock()
			.unwrap_or_else(PoisonError::into_inner)
			.iter()
			.filter_map(|row| row.content.upgrade())
			.collect()
	}

	#[cfg(test)]
	pub(in crate::model) fn cost(&self) -> (usize, usize, u64) {
		let frames = self.frames();
		let bytes = frames.iter().map(frame::Shared::written).sum();
		(frames.len(), bytes, self.size())
	}

	pub(super) fn size(&self) -> u64 {
		let frames = self.frames();
		self.bill.size()
			+ self
				.plain
				.lock()
				.unwrap_or_else(PoisonError::into_inner)
				.as_ref()
				.map_or(0, |plain| plain.bill.size())
			+ frames.iter().map(frame::Shared::content_size).sum::<u64>()
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
