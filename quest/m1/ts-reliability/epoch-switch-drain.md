# [M] A stitched switch keeps playing the old instance

## Goal

When `moq export ts --stitch` follows a newer epoch, the output keeps playing
the old instance until the new one's clock is acquired, then switches with the
marked program discontinuity it has today. A switch away from a healthy leg,
such as a redundant importer starting or restarting, leaves no hole in the
output. A switch after a failure costs only what the failed leg did not
deliver. Today `Export::followed`
(`rs/moq-mux/src/container/ts/export.rs`) replaces the whole export on a
switch, discarding the old leg's held backlog (up to one `--delay`) and
holding output while the new clock is acquired.

Out of scope: changing route order, timestamp continuity across epochs, and
hitless failover between importers.

## Plan

Decided 2026-10-26 (maintainer, importing
[moq-dev/moq#6877](https://github.com/moq-dev/moq/issues/6877)):

- Routing stays as decided on 2026-10-19: the newest epoch wins, so starting
  or restarting a backup importer is a switch. Rejected: a standby epoch that
  routes only when no live epoch remains (a second, publisher-declared
  selection policy ahead of epoch on the wire), and receiver-select in the CLI
  (a multi-broadcast `export ts`). Separate paths chosen by the subscriber
  stay documented in `doc/setup/redundancy.md`.
- The new instance opens a new jitter generation instead of a new export,
  reusing the generation machinery `rs/moq-mux/src/jitter.rs` already has for
  in-source timeline restarts: generations go out in turn, and the new clock
  is never ahead of a deadline already given out.
- The old instance's subscription keeps feeding its generation until the new
  generation is acquired, bounded by the `--follow` window, then stops.
  Rejected: playing out only what the buffer holds at the switch.
- Distinct epochs promise no timestamp relation, so the switch stays a program
  change: new PMT version, every PID flagged, PCR discontinuity. Do not align
  the legs' timestamps; that is the cross-importer alignment dropped on
  2026-10-19.
- The redundant failover integration test in
  `rs/moq-cli/tests/integration/export.rs` asserts that a switch away from a
  healthy leg adds no output gap when the replacement is usable within the
  follow, which the unimpaired run shows. Behind its bufferbloat impairment it
  asserts the marked switches only (see 2026-10-27).
- Fix inline: `doc/setup/redundancy.md` and `doc/bin/cli.md` say the switch
  is not hitless and that starting a backup causes it; say what a switch now
  costs.

Decided 2026-10-27 (maintainer, on
[moq-dev/moq#6919](https://github.com/moq-dev/moq/pull/6919)): the hard follow
bound cannot also guarantee no hole when the replacement becomes usable only
after the old input stops.

1. ✅ Keep the hard follow bound and limit the no-added-hole guarantee to
   replacements usable within it. Past the bound a hole is the honest result;
   old input is never kept alive without bound.
2. Keep old input feeding until the replacement is usable, even past the
   follow, which can hold its receive and processing cost indefinitely.

The real bufferbloat run's healthy holes (184.6 ms before, 173.7 ms and
783.7 ms reproduced) are this past-bound case, not a mux or clock fault. Its
160 kbit/s link is slower than either leg alone (about 250 kbit/s), and audio
outranks video, so while both legs are subscribed their audio fills the link:
traced group arrivals show neither leg's video completing a frame once the
replacement's audio flows, and the replacement's first keyframe arriving only
0.7 to 2.2 s after the follow stops the old input. Whether a hole appears then depends only on whether the
old tail outlasts that late acquisition. Unimpaired, the replacement commits
1.4 to 3 s after the follow starts, inside the bound, with no added gap.
Demoting the old leg's subscriber priority while the replacement acquires did
not get the replacement's video through sooner in this run, so it is not kept.

## Closes

- [#6877](https://github.com/moq-dev/moq/issues/6877) - a redundant ingest pair: a follower switches without emptying its delay (the standby-epoch ask is declined)
