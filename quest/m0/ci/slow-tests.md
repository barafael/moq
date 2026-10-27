# [M] Halve the slowest tests' CPU

## Goal

Before any source edit, freeze the slowest `ceil(N / 100)` test IDs from one full
normal `cargo nextest` run and list them in the quest's PR. Those same cases take
under half their baseline user-plus-system CPU on the same machine, with every
case and its coverage preserved. Per-test CPU is the test process's user plus
system time, not nextest's wall-clock duration. Missing or failed cases are not
imputed.

## Plan

Decided with the maintainer on 2026-10-26, from kixelated/rust2's
[test selection report](https://github.com/kixelated/rust2/pull/69): the slowest 1%
of tests make up 57% of what either package selection or a function-level selector
would run, so cutting them beats building a selector.

- Start with moq-mux's container import tests (137 s) and moq-net's integration tests
  (42 s).
- Prefer making a test cheaper: smaller fixtures, shared setup, simulated time instead
  of real waits, fewer redundant cases.
- Moving a test to nightly only follows this epic's rule: evidence and an explicit
  coverage tradeoff for each.

Public API: none.

### Checkpoint, 2026-10-27

[moq-dev/moq#6914](https://github.com/moq-dev/moq/pull/6914) retains useful test-only
changes but **does not complete this Goal**. No cases, durations, assertions, or
coverage were retired or moved to nightly.

- One full actual-nextest baseline before edits, Rust source
  `6f6a3ae21432a3df74d4c432451f3909da339bee` (main
  `88620648e0a236d28a01a4a797f58ae9ae720489`): 10,275 passed, 104 skipped,
  253.449958 test CPU-seconds. The frozen 103 IDs cost **137.834631 CPU-seconds**,
  so the target is **below 68.9173155 CPU-seconds**. The PR lists each ID and its
  user/system cost. Before source edits, the initial floor-rounded 102-row list
  was corrected once to `ceil(10,275 / 100) = 103`, using that same completed run.
- The final retained Rust source, `a6aa64383a1939f3bdffef1dd7e43460933b2c6c`,
  passed the same 10,275 tests with the same 104 skips. Every baseline ID and all
  103 frozen IDs reconciled. The cohort cost **116.833263 CPU-seconds**, a measured
  15.2366% reduction, still above the target. No base merge, new baseline, missing
  case imputation, or cohort reselection is included.
- Both runs used the same actual nextest command, native runner wrapper, local
  four-worker admission, groups, zero retries, timeouts, toolchain and lockfile.
  The wrapper recorded child `wait4` user plus system CPU, excluding wrapper and
  build CPU. Unchanged controls varied too; their differences are not attributed
  savings. The historical report's 103 tests / 289 of 501 CPU-seconds are
  motivation, not this run's baseline or savings.

Retained changes:

- The two MPEG-TS reset tests drain queued media on the exporter's existing
  deadlines and stop when idle. Their retention covers the entire original
  30-second leap plus eight PES packets of two 72 ms frames, rather than an
  arbitrary hour. Their combined CPU fell from 12.982402 to 0.178783 seconds;
  both phases must actually emit output. A per-phase-only retention trial failed
  the unsignalled-reset assertion and was discarded.
- The pure jitter `drift` and `feed` loops advance an existing explicit deadline
  queue synchronously. Undoing only these two clock drivers reproduces the
  original file exactly: every arrival/decode formula, frame count, duration,
  queueing spell, order, error branch, bound and assertion is unchanged. All 42
  jitter controls pass, including the full 24-hour drift cases.

Discarded experiments and remaining work:

- A cached lead median was dropped: the frame-rate benchmark was mostly within
  noise, and one cell regressed about 6%. No production jitter changes remain.
- A no-op containment shortcut for miss-verdict intersections was dropped. The
  sparse gap and shared-subscriber/range sweeps remained essentially unchanged;
  timed 4096-gap reads were about 2.796 ms before and 2.783 ms after. No net model
  or benchmark changes remain. The existing
  [incremental sparse-misses quest](/quest/m1/release-perf/sparse-misses.md)
  still owns its broader deferred-set rescans and O(log gaps) Goal; this PR does
  not complete it, range-cost, fill-linear, or drop-set.
- The same frozen 103 cases still need a further reduction to below 68.9173155
  CPU-seconds. Broader miss-ledger/lifecycle algorithms and coverage/fixture
  tradeoffs were not selected while the user was away. Keep this checkpoint a
  draft with the Goal unfinished rather than claim the target or tune it away.

Public API: none. Wire: none. Production behavior is unchanged.
