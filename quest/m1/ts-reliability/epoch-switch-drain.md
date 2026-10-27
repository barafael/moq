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
  `rs/moq-cli/tests/integration/export.rs` prints each switch's output gap
  without asserting it. Assert that a switch away from a healthy leg leaves
  no gap, behind a bufferbloat impairment.
- Fix inline: `doc/setup/redundancy.md` and `doc/bin/cli.md` say the switch
  is not hitless and that starting a backup causes it; say what a switch now
  costs.

### Implementation checkpoint (2026-10-27)

The private prototype keeps one export and jitter buffer, separates per-instance
input/program state, and adopts the new program metadata at generation release.
The buffer contains an active clock engine and at most one acquiring replacement,
using the existing clock math independently. A replacement commits only after an
actual usable video start (or the first usable item without video), retaining at
most one issued item per other track under backpressure. Commit flushes old decode
clocks and clamps the replacement against every old deadline already issued;
program metadata and clock statistics change only after the old tail drains.
Cancelling an unemitted replacement restores the old clock and keeps its data.
Per-program statistics retain their existing reset behavior; received-byte
accounting is unchanged.
The original source discarded 32 buffered frames before the replacement supplied
media; a paused-clock playback control also failed because the old subscription
stopped at the route switch. Both controls pass with the prototype. A delayed
replacement-video control reproduced a 1.133 s multi-track handoff hole before
two-phase acquisition; it now passes with unchanged PCR/frame cadence bounds for
zero, positive, and negative A/V leads. The 257 focused TS and jitter controls
also cover actual held-replacement cancellation, valid empty-catalog EOF drain,
cold same-epoch returns, old decode-clock flush, and independent active catalog
updates/timeline jumps while a replacement acquires.

The real relay/bufferbloat Goal is not complete. One original-source run reported
healthy switch gaps of 4.63/4.90 s and 8.20/8.16 s across its two viewers, and
failed-leg gaps of 3.52/3.52 s. That run did not record a steady-delivery statistic.
One candidate run, with the same impairment and switch ordering, reported healthy
gaps of 62.7/805.0 ms and 8.7/10.1 ms, and failed-leg gaps of 1.16/1.22 s.
The same-reader steady maxima were 35.8/43.8 ms. The healthy-gap assertion uses
that independent steady control plus the existing 25 ms PCR and 20 ms polling
grids, fixed before the candidate run. The second viewer's first switch fails it.
The margin and assertion must remain unchanged while that remaining handoff
interleaving is reproduced and fixed. These observations establish no matched
performance confidence or speedup claim.

The earlier candidate did not isolate the 805 ms boundary in transport logs;
the deterministic controls establish a related nominal-acquisition/startup
ownership failure, rather than claiming that exact transport trace as proven.
The changed source still needs the documented real-surface and clean-head gates.
No second full
export, clock alignment, routing policy, startup policy, or arbitrary buffer
budget is selected. The PR stays draft while this proof remains incomplete.

## Closes

- [#6877](https://github.com/moq-dev/moq/issues/6877) - a redundant ingest pair: a follower switches without emptying its delay (the standby-epoch ask is declined)
