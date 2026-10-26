# [S] moq-net compiles in Fuzz and send-depth

## Goal

Nightly's six Fuzz jobs and Tests (rs send-depth) compile `moq-net` again.

## Plan

Reproduced on 2026-10-26 at
[main e5feac72f6](https://github.com/moq-dev/moq/commit/e5feac72f6a5860f3c2b1d5f3c8e5aeade1c03c7)
with the pinned rust-overlay's Rust 1.101.0-nightly (2026-10-17) and
`just rs send-depth`. The original failure matches scheduled Nightly
[40192013303](https://github.com/moq-dev/moq/actions/runs/40192013303): proving
the listing task is `Send` traverses `run_publish`, `resume_publish`, the
authorization race, and `run_subscription` past Rust's default recursion limit.

The IETF operations now use the existing allocation-free `util::sendable`
proof boundary before nesting their futures: publishing a resumed subscription,
reading its answer, cancelling it, and reading a TRACK_STATUS response. The
compiler then reaches a separate failure in the concrete Lite driver state:

- `Driver` -> `SubscriberDriver` -> `kio::Tasks<SourceServe>` ->
  `kio::Tasks<TrackServeRun>` exceeds the same limit at `driver.rs`'s native
  `assert_send::<Driver>()`.
- A prototype erasing the outer source tasks with the existing `MaybeSendTask`
  moves the error to another branch: `Driver` -> `SubscriberDriver` -> the prefix
  map -> `AnnouncePrefix` -> `Announced` -> its held-route map. The prototype
  was restored; no additional boxes ship.
- The ASan `lite` fuzz target reproduces the concrete-driver failure too.
  The other five targets share that library build and remain unverified.

A maintainer decision remains on the Lite state's proof boundary. Recommend a
design pass that keeps its named machines and finds a shared ownership boundary
where the existing checked `SendCell` can stop repeated proofs. Alternatively,
erase the internal protocol driver behind its existing box, or split the state
more deeply. Establish that the chosen boundary proves within the default
limit, and measure any allocation or dispatch cost before adopting it. Do not
add unchecked `Send` implementations or raise the recursion limit.

Then run `just rs send-depth`, compile and smoke all six `just rs fuzz` targets,
and run the selected local gate. No public API or wire change is needed.
