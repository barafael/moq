use std::iter::Peekable;
use std::ops::RangeBounds;

use super::super::ring::Ring;
use super::SentPacket;

/// Ack-eliciting and ACK-only packets in separate rings.
#[derive(Default)]
pub(crate) struct Split {
	/// Bounded by the congestion window, plus loss probes that back off
	ack_eliciting: Ring<SentPacket>,
	/// Bounded by forgetting the oldest beyond [`super::MAX_ACK_ONLY`]
	ack_only: Ring<SentPacket>,
	in_flight: usize,
}

impl Split {
	pub(crate) fn insert(&mut self, pn: u64, packet: SentPacket) -> Option<SentPacket> {
		self.in_flight += usize::from(packet.size != 0);
		if packet.ack_eliciting {
			self.ack_eliciting.push(pn, packet);
			return None;
		}
		let mut forgotten = None;
		if self.ack_only.len() == super::MAX_ACK_ONLY {
			let (_, packet) = self.ack_only.pop_first().unwrap();
			self.in_flight -= usize::from(packet.size != 0);
			forgotten = Some(packet);
		}
		self.ack_only.push(pn, packet);
		forgotten
	}

	pub(crate) fn remove(&mut self, pn: u64) -> Option<SentPacket> {
		let packet = self.ack_eliciting.remove(pn).or_else(|| self.ack_only.remove(pn))?;
		self.in_flight -= usize::from(packet.size != 0);
		Some(packet)
	}

	pub(crate) fn get(&self, pn: u64) -> Option<&SentPacket> {
		self.ack_eliciting.get(pn).or_else(|| self.ack_only.get(pn))
	}

	pub(crate) fn last(&self) -> Option<u64> {
		self.ack_eliciting.last().max(self.ack_only.last())
	}

	pub(crate) fn has_in_flight(&self) -> bool {
		self.in_flight != 0
	}

	pub(crate) fn range(&self, range: impl RangeBounds<u64> + Clone) -> impl Iterator<Item = (u64, &SentPacket)> + '_ {
		Merge {
			a: self.ack_eliciting.range(range.clone()).peekable(),
			b: self.ack_only.range(range).peekable(),
		}
	}

	pub(crate) fn values_mut(&mut self) -> impl Iterator<Item = &mut SentPacket> + '_ {
		self.ack_eliciting.values_mut().chain(self.ack_only.values_mut())
	}

	pub(crate) fn into_values(self) -> impl Iterator<Item = SentPacket> {
		self.ack_eliciting.into_values().chain(self.ack_only.into_values())
	}

	#[cfg(test)]
	pub(crate) fn slots(&self) -> usize {
		self.ack_eliciting.len() + self.ack_only.len()
	}
}

/// Two packet-number-ordered iterators merged in order.
struct Merge<A: Iterator, B: Iterator> {
	a: Peekable<A>,
	b: Peekable<B>,
}

impl<'a, A, B> Iterator for Merge<A, B>
where
	A: Iterator<Item = (u64, &'a SentPacket)>,
	B: Iterator<Item = (u64, &'a SentPacket)>,
{
	type Item = (u64, &'a SentPacket);

	fn next(&mut self) -> Option<Self::Item> {
		match (self.a.peek(), self.b.peek()) {
			(Some(a), Some(b)) if b.0 < a.0 => self.b.next(),
			(Some(_), _) => self.a.next(),
			(None, _) => self.b.next(),
		}
	}
}
