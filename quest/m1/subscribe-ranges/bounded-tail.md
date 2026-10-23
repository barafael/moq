# [M] A copy that stopped at a request's end bound accepts its tail

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

Recommendation: make each stream's admitted end unambiguous, then append its
tail into the existing copy. A stream-specific end bound in the unpublished
lite-07 GROUP header would let FIN below that bound finish the source group,
and FIN exactly at it leave the copy resumable. An explicit final-versus-
bounded terminal is an alternative. This is a wire decision, not an approved
format change. Lite-06 also carries frame bounds and must remain compatible:
its solution needs negotiated signaling or request-identity correlation, not
an unconditional new header field. Changing only lite-07 would leave the Goal
unmet. Lite-05 and earlier round bounds outward to whole groups. Reuse IETF's
existing end bit/status rather than extending its wire format, including
emitting a genuine early end when the header did not promise one. Mirror lite
changes in JS and its draft.

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

Public API: none expected. Wire: lite boundary signaling needs a decision;
no format change is implemented by this checkpoint.

## Related

- [Compressed tracks](/quest/m1/compress/README.md) - its Ranges rule keeps a warm request at N, which this makes reachable
