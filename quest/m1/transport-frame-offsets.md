# [M] The transport may start a group mid-way

## Goal

A moq-lite hop may request and deliver a group starting at any frame k the
requester needs, whether or not it still holds the frames below k (route
continuation, hole fills, and front-eviction refills after the front was
evicted, with a frame-precise end), so a relay never refetches frames it does
not need. Applications still only ask for whole groups: no public API
starts at a frame. Rust and `@moq/net` both accept and cache such a tail.

## Plan

Decided 2026-10-25 (maintainer): remove frame-N starts from the API, as
[moq-dev/moq#6465](https://github.com/moq-dev/moq/pull/6465) did, but allow
them in the transport as an optimization.

- Draft: amend lite-07-wip so a subscriber may send any nonzero frame start,
  replacing "MUST send 0 unless it already holds every frame below this
  index". A front-eviction refill needs frames k and up after the frames
  below k were evicted, so holding the prefix cannot be the condition. Keep
  the `Frame End` rule.
- Widening is relay policy, not a wire rule: a relay asked for frame k of a
  group it holds none of widens the upstream request to frame 0 and cuts
  locally, so the cache stays usable by
  whole-group readers and a compressed group can inflate.
- A partial copy is the same group, never a new identity: `covering_group`
  already serves only copies that cover the start. A copy cut short at the back
  is marked incomplete so it never answers a whole-group fetch.
- JS accepts a GROUP with `frameStart > 0` (numbering from it, as Rust's
  `group::Producer::start_at` does) instead of refusing it as a protocol
  violation.
- A moq-transport stream that starts mid-group with no head is still dropped
  (2026-10-22 decision, reaffirmed 2026-10-25).
- The rule in `rs/moq-net/AGENTS.md` that banned partial groups was deleted
  2026-10-25.

Bound the cost a hostile peer can create: many distinct tail starts for one
group must not multiply its copies; coalesce to one copy per group.

Public API: none. Wire: lite-07-wip draft text only; the fields exist.

## Related

- [Front eviction](/quest/m1/unbounded-groups/front-eviction.md) - refills a front hole with a frame-precise range

### Implementation checkpoint

[moq-dev/moq#6922](https://github.com/moq-dev/moq/pull/6922) implements private
JS offset initialization before publication, exact forwarding/cache coverage,
and private Rust frame-end provenance. Public APIs and existing wire fields
remain unchanged. The full Goal is unfinished: retaining one cached head does
not bound storage held by earlier readers. The selected one-copy bound still
applies; no weaker ownership or resource policy has been selected.

Before fixing, the timed JS GROUP control passed at frame 0 and refused frame
5 and 9000 with PROTOCOL_VIOLATION. The actual Rust FetchServe control queued
start 2 for a cold request with start 2/end 3, rather than widening to 0.
Those controls now pass. Focused JS checks cover early/late mirrors, retained
frame limits, missing compression prefix, exact tail forwarding, and the
unchanged declared-end violation. Fifteen focused Rust controls pass, including
bounded-end cache misses, queued/taken range widening, and live-copy preservation.
These are focused checks, not final-head CI or full interop evidence.

A small deterministic held-reader control demonstrates the remaining cost in
both the inherited unbounded-end path and the bounded-end path. Eight tails
start at frames 8 through 1, with identical bytes for every matching frame
index, beside a live copy starting at 32. Each reader retains its tail unread.
The unbounded copies all end at 33; the bounded copies all end at 17.

| End | Distinct tail stores + live | Slot handles | Tail payload retained | Unique tail payload | Pool charge before/after identity-probe drop | After reader drop |
| --- | --- | --- | --- | --- | --- | --- |
| Unbounded | 8 + 1 | 2 | 233472 B | 32768 B | 266376 B / 266376 B | 40864 B |
| Exclusive 17 | 8 + 1 | 2 | 102400 B | 16384 B | 124456 B / 124456 B | 22144 B |

The live baseline charge is 2216 B. Distinct state channels, overlapping payloads,
and unchanged charge after identity probes are dropped establish reader-held
multiplication; Slot ownership alone does not prove the Goal. Existing
`Head::adopt_head` shares payload references but only joins a head that reaches
the target offset, refusing sparse gaps. The bounded copies ending at 17 do not
reach the live copy at 32. Coalescing must preserve existing public reader
outcomes, completion/error ownership, compression, and cache accounting.

The diagnostic below was inserted into `model::track::test`, run once through
the normal runner, then removed from executable source. It remains here so the
proof survives local scratch cleanup. The source and raw output are retained
under this worktree's `.tmp/held-tail-cost.rs` and
`.tmp/held-tail-overlap-cost.log`. It measures correctness and ownership, not
host performance or a new acceptance budget. SHA256 source:
`ea08934f00d65cb19443c9f0c90eec824ff6e2abc0fa347c587aef2564e33a76`;
raw output: `3c9efaf8a72a0a23e2f01c536e61a3ab0e5e4e52a35dd028eb6d73c7081c9eb9`.

```sh
nix develop --command just rs test -p moq-net --lib \
  -E 'test(=model::track::test::diagnostic_held_tail_storage)' \
  --success-output immediate
```

```rust
	#[moq_clock::test]
	#[cfg(feature = "async")]
	async fn diagnostic_held_tail_storage() {
		for bounded in [false, true] {
			let producer = track_producer("test", Info::timed());
			let pool = producer.broadcast.pool.clone();
			let dynamic = producer.dynamic();
			let consumer = producer.consume();
			let mut live = producer.create_group(group::Info { sequence: 0 }).unwrap();
			live.start_at(32).unwrap();
			live.write_frame(Timestamp::ZERO, vec![32; 1024]).unwrap();
			let baseline = pool.used();
			let mut held = Vec::new();
			let mut identities = Vec::new();
			for start in (1..=8).rev() {
				let options = group::Fetch { frame_start: start, ..Default::default() };
				let fetch = if bounded {
					consumer.fetch_range(0, FetchOptions { options, end: Some(17) })
				} else {
					consumer.fetch_group(0, options)
				};
				let request = dynamic.requested_fill_group().await.unwrap();
				let mut tail = request.accept(Info::timed()).unwrap();
				for frame in start..if bounded { 17 } else { 33 } {
					tail.write_frame(Timestamp::ZERO, vec![frame as u8; 1024]).unwrap();
				}
				tail.finish().unwrap();
				identities.push(tail.handle());
				held.push(fetch.await.unwrap());
			}
			for (i, left) in identities.iter().enumerate() {
				for right in &identities[i + 1..] { assert!(!left.is_clone(right)); }
				assert!(!left.is_clone(&live.handle()));
			}
			let before_identity_drop = pool.used();
			drop(identities);
			let only_readers_and_slot = pool.used();
			let cached_handles = producer.state.read().lookup[&0].copies().count();
			eprintln!("HELD_TAIL_COST bounded={bounded} requests=8 unique_tail_storages=8 live_storages=1 slot_handles={cached_handles} unread_payload={} unique_payload={} baseline_charge={baseline} charge_with_identities={before_identity_drop} charge_after_identity_drop={only_readers_and_slot}", if bounded { 100*1024 } else { 228*1024 }, if bounded { 16*1024 } else { 32*1024 });
			for (i, reader) in held.iter_mut().enumerate() {
				let frame = reader.read_frame().await.unwrap().unwrap();
				assert_eq!(frame.payload.len(), 1024);
				assert!(frame.payload.iter().all(|byte| *byte == (8-i) as u8));
				for expected in (9-i)..if bounded { 17 } else { 33 } {
					let frame = reader.read_frame().await.unwrap().unwrap();
					assert!(frame.payload.iter().all(|byte| *byte == expected as u8));
				}
			}
			drop(held);
			eprintln!("HELD_TAIL_COST bounded={bounded} charge_after_reader_drop={}", pool.used());
			live.abort(Error::Cancel).unwrap();
		}
	}
```
