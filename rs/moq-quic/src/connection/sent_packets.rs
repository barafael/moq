mod split;
mod tail;

use super::spaces::SentPacket;

pub(super) use split::Split as SentPackets;

/// ACK-only packets kept before the oldest is forgotten.
const MAX_ACK_ONLY: usize = 256;

/// The designs over packets carrying only a size and whether they elicit an ACK, for the benches.
#[cfg(feature = "fuzz")]
pub mod bench {
	use std::ops::RangeBounds;

	use super::SentPacket;
	use crate::Instant;

	macro_rules! wrap {
		($name:ident, $inner:ty) => {
			/// A `SentPackets` design.
			pub struct $name {
				packets: $inner,
				time_sent: Instant,
			}

			impl Default for $name {
				fn default() -> Self {
					Self {
						packets: Default::default(),
						time_sent: Instant::now(),
					}
				}
			}

			impl $name {
				/// Track `pn`, returning whether an ACK-only packet was forgotten.
				#[inline]
				pub fn insert(&mut self, pn: u64, size: u16, ack_eliciting: bool) -> bool {
					let packet = SentPacket {
						path_generation: 0,
						time_sent: self.time_sent,
						size,
						ack_eliciting,
						congestion: None,
						largest_acked: None,
						retransmits: Default::default(),
						stream_frames: Default::default(),
						bandwidth_probe: false,
					};
					self.packets.insert(pn, packet).is_some()
				}

				/// Stop tracking `pn`, returning whether it was tracked.
				#[inline]
				pub fn remove(&mut self, pn: u64) -> bool {
					self.packets.remove(pn).is_some()
				}

				/// The size of `pn`, if tracked.
				#[inline]
				pub fn get(&self, pn: u64) -> Option<u16> {
					self.packets.get(pn).map(|packet| packet.size)
				}

				/// The highest packet number tracked.
				#[inline]
				pub fn last(&self) -> Option<u64> {
					self.packets.last()
				}

				/// Tracked packet numbers and sizes in `range`, in order.
				#[inline]
				pub fn range(&self, range: impl RangeBounds<u64> + Clone) -> impl Iterator<Item = (u64, u16)> + '_ {
					self.packets.range(range).map(|(pn, packet)| (pn, packet.size))
				}

				/// Whether a tracked packet counts toward congestion control.
				#[inline]
				pub fn has_in_flight(&self) -> bool {
					self.packets.has_in_flight()
				}

				/// Drop every packet, returning how many were tracked.
				#[inline]
				pub fn drain(self) -> usize {
					self.packets.into_values().map(std::hint::black_box).count()
				}
			}
		};
	}

	wrap!(BTree, super::tail::BTree);
	wrap!(Chunked, super::tail::Chunked);
	wrap!(Split, super::split::Split);
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::Instant;
	use std::ops::{Bound, RangeBounds};

	macro_rules! design_tests {
		($module:ident, $design:ty) => {
			mod $module {
				use super::*;

				type SentPackets = $design;

				/// A peer that never ACKs leaves storage bounded by the packets still tracked, while
				/// ACK-only packets, padded or not, and loss probes keep going out.
				#[test]
				fn silent_peer_storage_is_bounded() {
					let mut m = SentPackets::default();
					let mut pn = 0;
					let mut in_flight = 0;
					let mut ack_only = 0;
					// A congestion window's worth of data.
					for _ in 0..32 {
						m.insert(pn, eliciting(1200));
						pn += 1;
						in_flight += 1;
					}
					// Loss probes go out ever more rarely between ACK-only packets.
					for probe in 0..16 {
						for i in 0..1000 << probe.min(6) {
							// Every other ACK-only packet is padded.
							let size = if i % 2 == 0 { 1200 } else { 0 };
							if let Some(forgotten) = m.insert(pn, packet(size)) {
								assert!(!forgotten.ack_eliciting);
								in_flight -= usize::from(forgotten.size != 0);
								ack_only -= 1;
							}
							in_flight += usize::from(size != 0);
							ack_only += 1;
							pn += 1;
						}
						m.insert(pn, eliciting(1200));
						pn += 1;
						in_flight += 1;
					}

					let live = m.range(..).count();
					assert_eq!(live, 32 + 16 + ack_only);
					assert_eq!(m.range(..).filter(|(_, p)| p.size != 0).count(), in_flight);
					println!("{}: live {live}, slots {}", stringify!($module), m.slots());
					assert!(m.slots() <= 2 * live + 64, "{} slots for {live} packets", m.slots());
				}

				#[test]
				fn ack_only_packets_are_capped() {
					let mut m = SentPackets::default();
					m.insert(0, eliciting(1200));
					for pn in 1..=MAX_ACK_ONLY as u64 {
						assert!(m.insert(pn, packet(0)).is_none());
					}
					let forgotten = m.insert(MAX_ACK_ONLY as u64 + 1, packet(7)).unwrap();
					assert_eq!(forgotten.size, 0);
					assert!(m.get(1).is_none());
					assert!(m.get(0).is_some());
					assert_eq!(m.range(..).count(), MAX_ACK_ONLY + 1);
				}

				#[test]
				fn insert_get_and_order() {
					let mut m = SentPackets::default();
					for pn in 3..=6 {
						m.insert(pn, eliciting(pn as u16 * 10));
					}
					assert_eq!(m.get(3).map(|p| p.size), Some(30));
					assert_eq!(m.get(6).map(|p| p.size), Some(60));
					assert!(m.get(2).is_none());
					assert!(m.get(7).is_none());
					assert_eq!(m.get(5).map(|p| p.size), Some(50));
					assert_eq!(m.last(), Some(6));
				}

				#[test]
				fn merged_order_and_gaps() {
					let mut m = SentPackets::default();
					m.insert(0, packet(0));
					m.insert(1, eliciting(1));
					m.insert(4, packet(4)); // 2 and 3 skipped
					m.insert(5, eliciting(5));
					m.insert(6, eliciting(6));
					m.insert(7, packet(7));
					assert!(m.get(2).is_none());
					assert_eq!(m.get(4).map(|p| p.size), Some(4));
					assert_eq!(
						range_of(&m, ..),
						vec![(0, 0), (1, 1), (4, 4), (5, 5), (6, 6), (7, 7)]
					);
					assert_eq!(range_of(&m, 2..=6), vec![(4, 4), (5, 5), (6, 6)]);
					assert_eq!(
						range_of(&m, (Bound::Excluded(4), Bound::Unbounded)),
						vec![(5, 5), (6, 6), (7, 7)]
					);
					assert_eq!(range_of(&m, 8..20), vec![]);
					assert_eq!(m.last(), Some(7));
				}

				#[test]
				fn remove_middle_and_front() {
					let mut m = SentPackets::default();
					for pn in 0..200 {
						let p = if pn % 5 == 0 { packet(pn as u16) } else { eliciting(pn as u16) };
						m.insert(pn, p);
					}
					for pn in (0..200).step_by(3) {
						assert_eq!(m.remove(pn).map(|p| p.size), Some(pn as u16));
						assert!(m.remove(pn).is_none());
					}
					let expected: Vec<_> = (0..200).filter(|pn| pn % 3 != 0).map(|pn| (pn, pn as u16)).collect();
					assert_eq!(range_of(&m, ..), expected);
					for pn in 0..100 {
						m.remove(pn);
					}
					assert_eq!(range_of(&m, ..), expected[expected.len() / 2..].to_vec());
					m.insert(500, eliciting(9));
					assert_eq!(m.get(500).map(|p| p.size), Some(9));
					assert_eq!(m.last(), Some(500));
				}

				#[test]
				fn values_mut_and_into_values() {
					let mut m = SentPackets::default();
					for pn in 0..4 {
						m.insert(pn, if pn == 2 { packet(pn as u16) } else { eliciting(pn as u16) });
					}
					m.remove(1);
					for v in m.values_mut() {
						v.size += 100;
					}
					assert_eq!(m.get(0).map(|p| p.size), Some(100));
					assert_eq!(m.get(2).map(|p| p.size), Some(102));
					let mut sizes: Vec<_> = m.into_values().map(|p| p.size).collect();
					sizes.sort();
					assert_eq!(sizes, vec![100, 102, 103]);
				}

				#[test]
				fn tracks_in_flight() {
					let mut m = SentPackets::default();
					assert!(!m.has_in_flight());
					m.insert(0, packet(0)); // size 0: not in flight
					assert!(!m.has_in_flight());
					m.insert(1, eliciting(1200));
					m.insert(2, packet(1200)); // padded
					assert!(m.has_in_flight());
					m.remove(1);
					assert!(m.has_in_flight());
					m.remove(2);
					assert!(!m.has_in_flight());
				}

				/// `(pn, size)` pairs from `range`.
				fn range_of<R: RangeBounds<u64> + Clone>(m: &SentPackets, r: R) -> Vec<(u64, u16)> {
					m.range(r).map(|(pn, v)| (pn, v.size)).collect()
				}
			}
		};
	}

	design_tests!(btree, tail::BTree);
	design_tests!(chunked, tail::Chunked);
	design_tests!(split_rings, split::Split);

	/// An ACK-only `SentPacket` identified by its `size`.
	fn packet(size: u16) -> SentPacket {
		SentPacket {
			path_generation: 0,
			time_sent: Instant::now(),
			size,
			ack_eliciting: false,
			congestion: None,
			largest_acked: None,
			retransmits: Default::default(),
			stream_frames: Default::default(),
			bandwidth_probe: false,
		}
	}

	/// An ack-eliciting `SentPacket` identified by its `size`.
	fn eliciting(size: u16) -> SentPacket {
		SentPacket {
			ack_eliciting: true,
			..packet(size)
		}
	}
}
