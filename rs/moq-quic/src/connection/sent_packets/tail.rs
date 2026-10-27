//! The sorted-map designs: one map, forgetting the oldest ACK-only packet only in the tail after
//! the newest ack-eliciting one, as before.

use std::collections::BTreeMap;
use std::ops::{Bound, RangeBounds};

use super::super::slots::Slots;
use super::SentPacket;

macro_rules! tail_design {
	($name:ident, $map:ty) => {
		#[derive(Default)]
		pub(crate) struct $name {
			packets: $map,
			in_flight: usize,
			/// The newest ack-eliciting packet number, after which ACK-only packets form the tail
			largest_ack_eliciting: Option<u64>,
			/// ACK-only packets present after `largest_ack_eliciting`
			tail: usize,
		}

		impl $name {
			pub(crate) fn insert(&mut self, pn: u64, packet: SentPacket) -> Option<SentPacket> {
				let mut forgotten = None;
				if packet.ack_eliciting {
					self.largest_ack_eliciting = Some(pn);
					self.tail = 0;
				} else {
					if self.tail == super::MAX_ACK_ONLY {
						let after = self.largest_ack_eliciting.map_or(Bound::Unbounded, Bound::Excluded);
						let oldest = self.packets.range((after, Bound::Unbounded)).next().unwrap().0;
						forgotten = self.remove(oldest);
					}
					self.tail += 1;
				}
				self.in_flight += usize::from(packet.size != 0);
				self.packets.insert(pn, packet);
				forgotten
			}

			pub(crate) fn remove(&mut self, pn: u64) -> Option<SentPacket> {
				let packet = self.packets.remove(pn)?;
				self.in_flight -= usize::from(packet.size != 0);
				if !packet.ack_eliciting && self.largest_ack_eliciting.is_none_or(|largest| pn > largest) {
					self.tail -= 1;
				}
				Some(packet)
			}

			pub(crate) fn get(&self, pn: u64) -> Option<&SentPacket> {
				self.packets.get(pn)
			}

			pub(crate) fn last(&self) -> Option<u64> {
				self.packets.last()
			}

			pub(crate) fn has_in_flight(&self) -> bool {
				self.in_flight != 0
			}

			pub(crate) fn range(
				&self,
				range: impl RangeBounds<u64> + Clone,
			) -> impl Iterator<Item = (u64, &SentPacket)> + '_ {
				self.packets.range(range)
			}

			pub(crate) fn values_mut(&mut self) -> impl Iterator<Item = &mut SentPacket> + '_ {
				self.packets.values_mut()
			}

			pub(crate) fn into_values(self) -> impl Iterator<Item = SentPacket> {
				self.packets.into_values()
			}

			#[cfg(test)]
			pub(crate) fn slots(&self) -> usize {
				self.packets.slots()
			}
		}
	};
}

/// `BTreeMap` behind the same interface as [`Slots`].
#[derive(Default)]
pub(crate) struct Tree(BTreeMap<u64, SentPacket>);

impl Tree {
	fn insert(&mut self, pn: u64, packet: SentPacket) {
		self.0.insert(pn, packet);
	}
	fn remove(&mut self, pn: u64) -> Option<SentPacket> {
		self.0.remove(&pn)
	}
	fn get(&self, pn: u64) -> Option<&SentPacket> {
		self.0.get(&pn)
	}
	fn last(&self) -> Option<u64> {
		self.0.last_key_value().map(|(&pn, _)| pn)
	}
	fn range(&self, range: impl RangeBounds<u64>) -> impl Iterator<Item = (u64, &SentPacket)> + '_ {
		self.0.range(range).map(|(&pn, packet)| (pn, packet))
	}
	fn values_mut(&mut self) -> impl Iterator<Item = &mut SentPacket> + '_ {
		self.0.values_mut()
	}
	fn into_values(self) -> impl Iterator<Item = SentPacket> {
		self.0.into_values()
	}
	#[cfg(test)]
	fn slots(&self) -> usize {
		self.0.len()
	}
}

tail_design!(BTree, Tree);
tail_design!(Chunked, Slots<SentPacket>);
