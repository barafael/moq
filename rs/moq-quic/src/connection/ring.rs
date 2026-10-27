use std::collections::VecDeque;
use std::ops::{Bound, RangeBounds};

/// Entries keyed by strictly increasing packet numbers, stored by position rather than by number.
///
/// A gap in the numbers costs nothing. A removed entry leaves a hole until every entry before it
/// is gone too, so storage spans from the oldest present entry to the newest.
pub(super) struct Ring<T> {
	/// Never starts or ends with a hole.
	entries: VecDeque<(u64, Option<T>)>,
}

impl<T> Default for Ring<T> {
	fn default() -> Self {
		Self {
			entries: VecDeque::new(),
		}
	}
}

impl<T> Ring<T> {
	/// Append `value` at `pn`, which must exceed every packet number pushed before.
	pub(super) fn push(&mut self, pn: u64, value: T) {
		debug_assert!(self.entries.back().is_none_or(|&(last, _)| pn > last));
		self.entries.push_back((pn, Some(value)));
	}

	/// The oldest present entry.
	pub(super) fn first(&self) -> Option<(u64, &T)> {
		let (pn, value) = self.entries.front()?;
		Some((*pn, value.as_ref().expect("no leading hole")))
	}

	/// The newest packet number present.
	pub(super) fn last(&self) -> Option<u64> {
		self.entries.back().map(|&(pn, _)| pn)
	}

	/// Remove and return the oldest present entry.
	pub(super) fn pop_first(&mut self) -> Option<(u64, T)> {
		let (pn, value) = self.entries.pop_front()?;
		self.trim_front();
		Some((pn, value.expect("no leading hole")))
	}

	pub(super) fn get(&self, pn: u64) -> Option<&T> {
		self.entries[self.position(pn).ok()?].1.as_ref()
	}

	pub(super) fn remove(&mut self, pn: u64) -> Option<T> {
		let index = self.position(pn).ok()?;
		let value = self.entries[index].1.take()?;
		if index == 0 {
			self.trim_front();
		} else if index + 1 == self.entries.len() {
			let holes = self.entries.iter().rev().take_while(|(_, value)| value.is_none()).count();
			self.entries.truncate(self.entries.len() - holes);
		}
		Some(value)
	}

	pub(super) fn is_empty(&self) -> bool {
		self.entries.is_empty()
	}

	/// Present entries in `range`, in increasing packet-number order.
	pub(super) fn range(&self, range: impl RangeBounds<u64>) -> impl Iterator<Item = (u64, &T)> + '_ {
		let start = match range.start_bound() {
			Bound::Included(&pn) => self.position(pn).unwrap_or_else(|index| index),
			Bound::Excluded(&pn) => match self.position(pn) {
				Ok(index) => index + 1,
				Err(index) => index,
			},
			Bound::Unbounded => 0,
		};
		let end = range.end_bound().cloned();
		self.entries
			.range(start..)
			.take_while(move |&&(pn, _)| match end {
				Bound::Included(end) => pn <= end,
				Bound::Excluded(end) => pn < end,
				Bound::Unbounded => true,
			})
			.filter_map(|(pn, value)| Some((*pn, value.as_ref()?)))
	}

	/// Mutably iterate present entries in increasing packet-number order.
	pub(super) fn values_mut(&mut self) -> impl Iterator<Item = &mut T> + '_ {
		self.entries.iter_mut().filter_map(|(_, value)| value.as_mut())
	}

	/// Consume the ring, yielding present entries in increasing packet-number order.
	pub(super) fn into_values(self) -> impl Iterator<Item = T> {
		self.entries.into_iter().filter_map(|(_, value)| value)
	}

	/// Entries plus holes held, which bounds the memory used.
	pub(super) fn len(&self) -> usize {
		self.entries.len()
	}

	/// Drop the holes left at the front, in place.
	fn trim_front(&mut self) {
		let holes = self.entries.iter().take_while(|(_, value)| value.is_none()).count();
		if holes != 0 {
			// Dropped in place: a hole moved out would still copy a whole entry.
			self.entries.drain(..holes);
		}
	}

	/// The index holding `pn`, or the index of the first entry above it.
	fn position(&self, pn: u64) -> Result<usize, usize> {
		let (Some(&(first, _)), Some(&(last, _))) = (self.entries.front(), self.entries.back()) else {
			return Err(0);
		};
		if pn < first {
			return Err(0);
		}
		if pn > last {
			return Err(self.entries.len());
		}

		// Numbers are distinct and increasing, so `pn` sits no further from either end than its
		// numeric distance. Dense numbering, the usual case, makes the far bound exact.
		let max = self.entries.len() - 1;
		let mut high = usize::try_from(pn - first).map_or(max, |distance| distance.min(max));
		if self.entries[high].0 == pn {
			return Ok(high);
		}
		let mut low = usize::try_from(last - pn).map_or(0, |distance| max.saturating_sub(distance));
		if self.entries[low].0 == pn {
			return Ok(low);
		}

		// The entry at `high` is above `pn`, and the one at `low` is below it.
		while low + 1 < high {
			let mid = low + (high - low) / 2;
			match self.entries[mid].0.cmp(&pn) {
				std::cmp::Ordering::Less => low = mid,
				std::cmp::Ordering::Equal => return Ok(mid),
				std::cmp::Ordering::Greater => high = mid,
			}
		}
		Err(high)
	}
}
