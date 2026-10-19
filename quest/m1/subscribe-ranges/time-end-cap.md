# [M] One speculative group past a time end

## Goal

A subscriber whose range ends at `Time(pts)` receives at most one group past that
end at the live edge, however many groups the publisher opens before their first
frames land. Today an unresolved time end covers every sequence through `u64::MAX`,
so each group handed out before its first frame can leak past the end; the docs
state that real bound (`track::Anchor::Time`, `doc/concept/moq-lite.md`).

## Plan

Found by Codex on #5929 and reproduced there: range `time(3000)..time(3500)` with
three groups opened before any frame delivers all three, not one.

- `Window` caps an unresolved time end at the newest held group known to start at or
  before pts, plus the first unstarted group above it, instead of `u64::MAX`.
  `settle` recomputes the cap as groups start.
- Raising the cap must wake readers: register on the speculative group's start
  (`group::Producer::poll_started`) from both `Cursor::poll_recv_group` and
  `poll_next_group`, so a parked group past the cap is re-offered once the group
  before it starts, without waiting on the next arrival.
- A held group that never shows a start (a copy joined mid-group) must not pin the
  cap: a later group starting at or before pts lifts it past.
- Datagram-only timed tracks: datagrams are not in `lookup`, so a time end on them
  never resolves and the cap would stay at the start group, dropping every later
  datagram. Resolve datagram starts too, or leave datagram reads uncapped.

Regression test: the #5929 repro, plus the wake (a parked group re-offered when its
predecessor's first frame lands) under mock time.

## Related

- [Dynamic range requests](/quest/m1/subscribe-ranges/dynamic.md) - where time anchors are resolved for a `Dynamic` handler
