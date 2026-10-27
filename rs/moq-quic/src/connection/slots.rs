use std::collections::VecDeque;
use std::ops::{Bound, RangeBounds};

/// Entries keyed by increasing packet numbers, in chunks of [`CHUNK`] consecutive slots.
///
/// A port of quinn-rs/quinn#2942. An emptied chunk is freed wherever it sits, so a hole costs at
/// most one chunk's slots.
pub(super) struct Slots<T> {
	chunks: VecDeque<Chunk<T>>,
}

const CHUNK: usize = 64;

struct Chunk<T> {
	/// The packet number of the first slot.
	offset: u64,
	slots: Box<[Option<T>; CHUNK]>,
	/// Bit `i` is set when slot `i` holds an entry.
	mask: u64,
}

impl<T> Chunk<T> {
	fn slot(&self, pn: u64) -> Option<usize> {
		let slot = usize::try_from(pn.checked_sub(self.offset)?).ok()?;
		(slot < CHUNK).then_some(slot)
	}
}

impl<T> Default for Slots<T> {
	fn default() -> Self {
		Self { chunks: VecDeque::new() }
	}
}

impl<T> Slots<T> {
	/// Insert `value` at `pn`, which must exceed every packet number inserted before.
	pub(super) fn insert(&mut self, pn: u64, value: T) {
		if let Some(chunk) = self.chunks.back_mut()
			&& let Some(slot) = chunk.slot(pn)
		{
			debug_assert!(chunk.mask >> slot == 0, "insertions must increase");
			chunk.slots[slot] = Some(value);
			chunk.mask |= 1 << slot;
			return;
		}
		let mut slots = Box::new([const { None }; CHUNK]);
		slots[0] = Some(value);
		self.chunks.push_back(Chunk {
			offset: pn,
			slots,
			mask: 1,
		});
	}

	pub(super) fn first(&self) -> Option<(u64, &T)> {
		let chunk = self.chunks.front()?;
		let slot = chunk.mask.trailing_zeros() as usize;
		Some((chunk.offset + slot as u64, chunk.slots[slot].as_ref()?))
	}

	pub(super) fn last(&self) -> Option<u64> {
		let chunk = self.chunks.back()?;
		Some(chunk.offset + u64::from(63 - chunk.mask.leading_zeros()))
	}

	pub(super) fn pop_first(&mut self) -> Option<(u64, T)> {
		let pn = self.first()?.0;
		Some((pn, self.remove(pn)?))
	}

	pub(super) fn get(&self, pn: u64) -> Option<&T> {
		let chunk = &self.chunks[self.chunk(pn)?];
		chunk.slots[chunk.slot(pn)?].as_ref()
	}

	pub(super) fn remove(&mut self, pn: u64) -> Option<T> {
		let index = self.chunk(pn)?;
		let chunk = &mut self.chunks[index];
		let slot = chunk.slot(pn)?;
		let value = chunk.slots[slot].take()?;
		chunk.mask &= !(1 << slot);
		if chunk.mask == 0 {
			self.chunks.remove(index);
		}
		Some(value)
	}

	pub(super) fn is_empty(&self) -> bool {
		self.chunks.is_empty()
	}

	/// Present entries in `range`, in increasing packet-number order.
	pub(super) fn range(&self, range: impl RangeBounds<u64>) -> impl Iterator<Item = (u64, &T)> + '_ {
		let low = match range.start_bound() {
			Bound::Included(&pn) => pn,
			Bound::Excluded(&pn) => pn.saturating_add(1),
			Bound::Unbounded => 0,
		};
		let high = match range.end_bound() {
			Bound::Included(&pn) => pn.saturating_add(1),
			Bound::Excluded(&pn) => pn,
			Bound::Unbounded => u64::MAX,
		};
		let start = match self.chunks.front() {
			Some(chunk) if low < chunk.offset + CHUNK as u64 => 0,
			_ => self.chunks.partition_point(|chunk| chunk.offset <= low).saturating_sub(1),
		};
		self.chunks
			.range(start..)
			.take_while(move |chunk| chunk.offset < high)
			.flat_map(move |chunk| {
				let from = low.saturating_sub(chunk.offset).min(CHUNK as u64) as u32;
				let mut mask = chunk.mask.checked_shr(from).map_or(0, |mask| mask << from);
				std::iter::from_fn(move || {
					if mask == 0 {
						return None;
					}
					let slot = mask.trailing_zeros() as usize;
					mask &= mask - 1;
					Some((chunk.offset + slot as u64, chunk.slots[slot].as_ref()?))
				})
			})
			.take_while(move |&(pn, _)| pn < high)
	}

	pub(super) fn values_mut(&mut self) -> impl Iterator<Item = &mut T> + '_ {
		self.chunks
			.iter_mut()
			.flat_map(|chunk| chunk.slots.iter_mut().filter_map(Option::as_mut))
	}

	pub(super) fn into_values(self) -> impl Iterator<Item = T> {
		self.chunks
			.into_iter()
			.flat_map(|chunk| chunk.slots.into_iter().flatten())
	}

	/// Slots held, which bounds the memory used.
	#[cfg(test)]
	pub(super) fn slots(&self) -> usize {
		self.chunks.len() * CHUNK
	}

	/// The index of the chunk that may hold `pn`, checking the newest and oldest first.
	fn chunk(&self, pn: u64) -> Option<usize> {
		let last = self.chunks.len().checked_sub(1)?;
		if pn >= self.chunks[last].offset {
			return Some(last);
		}
		if pn < self.chunks[0].offset + CHUNK as u64 {
			return Some(0);
		}
		Some(self.chunks.partition_point(|chunk| chunk.offset <= pn).saturating_sub(1))
	}
}
