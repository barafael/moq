# [XL] A copy that stopped at a request's end bound accepts its tail

## Goal

A group received for a request whose end falls mid-group stays open to the
rest of that group. When later demand asks the same sequence past that end,
the tail is accepted instead of rejected as `Duplicate`, so a second
subscriber on the same relay reads the frames it asked for. This holds for
plain and compressed tracks on every version that carries frame bounds.

## Plan

Reproduced on `ee1111a91b` through one relay: subscriber A reads group 5
frames 0..3, then subscriber B asks for 3..5. Local readers get all requested
frames, but a relayed lite-07 copy loses the tail on both plain and compressed
tracks. Lite-06 plain and draft-20, 21, and 22 lose it too. The deterministic
reproductions and early/exact-end controls are in
`rs/moq-net/tests/integration/bounded_tail.rs`.
The tail tests assert the intended behavior and currently fail; they are not
ignored or changed to require the bug.

`GroupRecv` (`rs/moq-net/src/lite/subscriber.rs`) always calls `group.finish()`
on FIN. `claim_sequence` and `covering_group` (`model/track.rs`) then treat
the copy as covering every later request from its first frame. IETF
`GroupIngest` similarly collapses a bare FIN and an explicit END_OF_GROUP into
`Ended::Group`, even though its publisher clears `GroupFlags.has_end` when
serving a bounded slice. JS has the same unconditional close in its lite
subscriber.

Settle the stream-boundary distinction before changing ownership. Lite-06/07's
GROUP header carries only subscription ID, sequence, and starting frame. It
does not carry the request's end or revision. After an update widens 3 to 5,
the same header, frames 0..3, and FIN can be:

- A stream admitted under the old end of 3, with more source frames to come.
- A stream admitted under the new end of 5, whose source really ends at 3.

Latching the receiver's current end when the header arrives cannot distinguish
these cases: control and data streams can arrive out of order. Remembering
every historical end has unbounded cost without a retirement signal. Never
declare an early FIN to have reached a requested end.

Decided by the maintainer (2026-10-23), superseding the earlier park:

- Lite-07-wip carries an explicit stream-level indication of bounded versus
  final, updated in `drafts/draft-lcurley-moq-lite.md`. It is unpublished, so
  its framing changes freely. Proposed shape: a GROUP `Frame End` (exclusive,
  0 when the stream runs to the group's end). A FIN below it ends the group;
  a FIN at it ends only the stream, and the publisher serves the tail on a
  later stream for the same sequence once the end widens. FRAME has no type
  byte, so a trailing terminal message would need new framing.
- Lite-06 has no such field: it falls back to request correlation, a fresh
  request identity for each end revision, so a stream's bound is its
  request's.
- IETF reuses `has_end` / END_OF_GROUP, with no new wire, including emitting
  a genuine early end that the header did not promise.
- `js/net` mirrors the lite-07 change.
- Too big for one PR: split rather than land part of it. Lite-05 and earlier
  round bounds outward to whole groups and need nothing.

Proposed split, awaiting the maintainer, in order:

1. Lite-07 end to end: the draft, the GROUP codec in Rust and JS, Rust and JS
   publishers labelling capped streams and serving the tail after the end
   widens, and the Rust subscriber appending into the cached copy (append
   ownership below). JS subscribers never ask for frame bounds, so they only
   decode the field.
2. Lite-06 request correlation on top of 1's append path.
3. IETF `has_end` / END_OF_GROUP on top of 1's append path, with the
   Object-ID/ordinal handling.

For append ownership, transfer one private writer permission into cache-owned
group state between streams. Never park a producer/parent clone or permit two
writers. Resume atomically at the committed frame boundary, preserve the
compressed prefix and inflater, and leave cached payload subject to normal
eviction, abort, and route teardown. A replayed prefix must be decoded and
discarded without writing it twice. Replacement is less attractive: warm
compressed demand intentionally starts at N, so replacing a copy also needs
its prefix and correct cache coverage, fetch, and existing-reader recovery.

Verify plain/compressed tails, real ends before/at/after the cap, empty tails,
duplicate concurrent streams, reset/eviction/teardown, and repeated demand
widening. Add a scripted case where the update overtakes an earlier GROUP
header, and the converse where a real early FIN follows the update. Neither
test may depend on sleeps or accepting a missing tail. Exercise the matching
JS path and cross-language interop after the boundary contract is settled.

The publisher half is missing too: the lite publisher marks a capped group
served and its cursor moves past it, so a widening SUBSCRIBE_UPDATE never
reaches the tail even on one hop, and the draft forbids a second Group Stream
for a sequence on one subscription.

Public API: none expected. Wire: lite-07 GROUP gains the bounded/final
indication above; no format change is implemented by this checkpoint.

## Related

- [Compressed tracks](/quest/m1/compress/README.md) - its Ranges rule keeps a warm request at N, which this makes reachable
