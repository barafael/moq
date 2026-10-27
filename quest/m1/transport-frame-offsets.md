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

### Canonical-owner prototype checkpoint

The private Rust prototype activates an indexed content owner when a second
transport delivery would coexist with the first. Ordinary single-copy whole
and batch writes retain their existing representation. Activation imports the
existing payloads without copying and attaches the exact in-flight `FrameBuf`,
so an already-streaming reader and writer keep their progress. New deliveries
fork their own received counter while sharing immutable content, publication
and the origin's admission. Contradictory bytes use `ProtocolViolation`.

A `TrackState`-scoped weak index lets a refill discover an owner still retained
by older tails after its Slot was removed. Owners remove their weak entries;
the index is owned by the live owners, so dead history does not accumulate.
Explicit live-group recreation replaces the sequence's indexed incarnation.
A different track instance does not inherit it. Delivery logs, bounds, FIN,
errors, timestamps and public arrival records remain separate. IETF Object IDs
have their own key space instead of being mistaken for dense frame ordinals.

Intrinsic `FrameBuf` charges follow the shared storage and each progress view,
including streamed readers that outlive the delivery's partial slot. A
payload-only `Charge::transfer` moves old bytes without recording them as new
writes. Fresh attachment and already-canonical transfer have controls. Deferred
log entries hold weak row handles; a non-polling cursor cannot keep a canonical
owner alive through retired log storage. Handoff heads retain the content owner.

The first eager prototype exposed real ownership and accounting integration
failures: 466/491 existing group/track tests passed initially, then 482/491
after ownership corrections. Its sources and raw results remain in
`.tmp/eager-content-prototype/`, `.tmp/content-model-tests.log` and
`.tmp/content-model-tests-2.log`. It was replaced by lazy activation rather than
relaxing cache-pressure thresholds. Two refill fixtures had different bytes for
the same fixed incarnation; only those payloads were made consistent, preserving
the bounds, lifetime and end-refusal assertions.

The lazy implementation passes 508/508 strict frame, group and track controls,
including the following new outer controls:

- Eight distinct tail starts with all earlier readers held, at 0, 1 and 1024 B
  per frame, both bounded and unbounded. Matching nonempty frame payloads have
  the same physical pointer. Dropping identity probes changes no charge.
- Actual Slot removal with an older reader-held tail, followed by a refill
  sharing its owner; explicit live recreation accepts different bytes and uses
  a separate owner.
- Activation while the original frame and streaming reader are already open.
  Overlap credits no progress to that delivery. One abort leaves the other
  producer valid and admitted. Two completions retain distinct arrival times
  10 ms apart under the deterministic clock while sharing the same payload.

The nine delivery handles remain distinct. Their logs and arrival metadata are
charged separately; this proves one canonical content store, not a constant
bound on all delivery bookkeeping or arbitrary caller-retained public Frames.
The bounded case also retains the live frame at 32 beside tail frames 1..16.

| Payload/frame | End | Canonical rows | Unique payload | Shared charge | Delivery charge | Discovery charge | Total |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 0 B | Unbounded | 32 | 0 B | 10368 B | 34272 B | 136 B | 44776 B |
| 0 B | Exclusive 17 | 17 | 0 B | 5688 B | 22272 B | 136 B | 28096 B |
| 1 B | Unbounded | 32 | 32 B | 10400 B | 34272 B | 136 B | 44808 B |
| 1 B | Exclusive 17 | 17 | 17 B | 5705 B | 22272 B | 136 B | 28113 B |
| 1024 B | Unbounded | 32 | 32768 B | 43136 B | 34272 B | 136 B | 77544 B |
| 1024 B | Exclusive 17 | 17 | 17408 B | 23096 B | 22272 B | 136 B | 45504 B |

On this x86_64 build the shared frame content allocation is 184 B, a progress
view is 56 B, and admission bookkeeping is 32 B. The empty-payload cases expose
the metadata independently of payload savings. These are ownership and charge
measurements, not quiet-host timing or allocator-overhead claims.

```sh
nix develop --command just rs test -p moq-net --lib \
  -E 'test(model::frame::tests) | test(model::group::test) | test(model::track::test)' \
  --no-fail-fast --success-output immediate
```

Raw output: `.tmp/lazy-content-model-tests-2.log`, SHA256
`76e5c37663d9529589e0119fa817cf3c064cfd4cc436a099f56bf6055bdccbda`.
`just fix` also passed. These focused results precede this checkpoint's final
integration gates; the full CI and interop pass on 5796 remain separate evidence.

The Goal remains unfinished pending typed sparse/relay and compressed raw/plain
outer preservation controls, actual-main prerequisite integration, final clean
committed-head gates and real surface proof. Actual compressed transport ingress
must share stored raw slices as received, with one lazy inflated copy. Local
plaintext-to-DEFLATE producer scratch is a distinct representation; it is not
claimed to share raw wire storage. Compressed transport fetches must continue
widening to start 0/end None, and a plain reader without the prefix must still
refuse. No public API, wire, resource-budget or maintainer-policy choice has been
selected by this checkpoint.

### Storage-lifetime preservation checkpoint

The later implementation supersedes the 2420 prototype's ownership details.
Canonical frame storage now owns its group identity, while the group's index
holds weak frame records. Completed delivery entries retain shared frame owners;
their owner cell can be populated during activation without replacing payloads
that lock-free readers already see. A streamed reader keeps the same identity
through its existing buffer. Last storage release removes its weak record, so
neither retired entries nor frame readers can hide retained bytes from a refill.
The index retains no dead weak history and introduces no new union-wide limits.
Existing per-delivery limits remain enforced.

A deterministic evicted-streaming-reader control first failed because its weak
group identity vanished while its frame buffer still existed. It now passes,
reusing the same physical payload on refill while the old reader still returns
`Evicted`. A separate control accepts an existing valid 8192-frame tail and a
disjoint valid one-frame prefix, preserving their existing per-delivery limits.

An independent review found that a captured inflater job could outlive delivery
A's release when delivery B kept the shared Plain alive. The existing
`inflation_is_shared_and_stops_at_release` control was extended and reproduced
that failure. A compressed delivery now has its own release guard. Each inflation
frame and get synchronizes with that guard; releasing A waits for its current
frame and preserves A's error while B can still read and inflate. The guard's
metadata is charged to the active delivery. This changes neither the shared
inflation representation nor the existing caller ownership of completed bytes.

Typed object controls preserve IDs across different delivery ordinals, sealed
timestamps and delivery-local terminal metadata. The compressed ingress control
uses independent received buffers, verifies raw payload pointer reuse without
recompression, preserves the missing-prefix refusal and different arrival times,
inflates plain bytes once, and keeps start-0/end-None widening for repeated tail
requests. Local plaintext compression scratch remains a separate producer path.

The full strict frame/group/track cohort passes 512/512. `just fix` passes.
Source includes the actually landed request and Path prerequisites through a
normal merge of main `c764acebb90b5c102a8ea90207dd125ae4fd326b`; no rebase.
The subsequent IETF version-removal integration and final full gates remain.

The updated x86_64 metadata sizes are content 216 B, progress view 56 B and
admission bookkeeping 32 B. Ownership charges from the retained-reader control:

| Payload/frame | End | Canonical rows | Unique payload | Shared charge | Delivery charge | Discovery charge | Total |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 0 B | Unbounded | 32 | 0 B | 8304 B | 36448 B | 136 B | 44888 B |
| 0 B | Exclusive 17 | 17 | 0 B | 5064 B | 23296 B | 136 B | 28496 B |
| 1 B | Unbounded | 32 | 32 B | 8336 B | 36448 B | 136 B | 44920 B |
| 1 B | Exclusive 17 | 17 | 17 B | 5081 B | 23296 B | 136 B | 28513 B |
| 1024 B | Unbounded | 32 | 32768 B | 41072 B | 36448 B | 136 B | 77656 B |
| 1024 B | Exclusive 17 | 17 | 17408 B | 22472 B | 23296 B | 136 B | 45904 B |

Raw outputs and SHA256:

- Full focused model cohort: `.tmp/weak-owner-model-tests.log`,
  `65b5eba4bd4a0c6d95536806469c8ede169cb5f6112ae11a2d638e094590fbf4`.
- Frame-reader negative: `.tmp/frame-reader-owner-before-2.log`,
  `41b552539fe590464c5040fbc93e8fc0597214cb73bfaa630fce1124b06c7005`.
- Frame-reader/union positive: `.tmp/weak-frame-owner-controls.log`,
  `13639d690f523102d340f5f4106f2b8ed9eb9a9e76184452bb470006cf716f4c`.
- Shared late-inflater negative: `.tmp/shared-late-inflate-before.log`,
  `ad2524de1e81d2a5e01ee02707585401ae15d428653f21a40392e7d010452c99`.
- Late-inflater/typed/compressed positives:
  `.tmp/canonical-representation-controls.log`,
  `44a2868fc82a01871af38327cc13697450988df42c9c379bbc946796bd80bec1`.

The Goal is still unfinished. Read-only review found a direct live Lite GROUP
path that creates a public whole group before applying the received frame start,
then resets the owner when replacing an earlier positive-start copy. The next
control must exercise actual GroupRecv with held readers and distinct tail
starts, followed by a private transport claim that knows the offset before
publication. Public explicit group recreation must retain its separate owner.
Ordinary pre-existing cross-route duplication is not itself evidence of this
specific tail-amplification bug. Final readiness still needs this direct ingress
proof, current-main integration, clean committed-head gates and the real surface.
