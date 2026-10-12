# [M] rust-js spike

## Goal

A report on running [rust-js](https://github.com/rust-js-lang/rust-js) over
moq-net's lite codec and model layer, and the maintainer's choice of the
translator's foundation from it: Charon in a new repo (the default), an own
THIR driver, contributing to rust-js, or forking it. The PR rewrites
[the rs2ts repo quest](/quest/m1/rs2ts/repo.md) for the chosen foundation.

## Plan

A planning quest: the report and the rewritten quests are the output, not
production code. Keep spike code on a scratch branch or in the report's
artifact. It reports facts; no pass/fail bar is set in advance.

Report, for the codec (`coding/` plus the lite message types) and the model
(`model/` plus kio):

- What translates unmodified, what fails, and the source changes needed,
  per construct: close-on-last-drop through `Arc` (and `Arc::strong_count`
  logic in `model/resume.rs`), `Weak`, kio's wakers and RAII guards,
  atomics, `BTreeMap::range`, maps keyed by `u64` or structs, the nested
  `Option`s, `Arc<dyn Expiry>`, and the `unsafe` frame storage.
- Gzip size and per-frame CPU of the generated codec against js/net's
  hand-written one. Bun or node numbers are a proxy for the browser; label
  them so.
- What it takes to get the shapes the line decided: `U64` as two `u32`
  halves instead of BigInt, checked integer arithmetic instead of wrapping,
  TypeScript output, and no async translation. Say whether each is a config
  option, an upstream patch, or a fork.
- rust-js's churn, maintenance, and license, and Charon's for comparison:
  Charon has no crates.io release, and its LLBC format breaks about five
  times a week.

The prior Charon evidence is the 2026-09-27 spike and the closed prototype
[#5049](https://github.com/moq-dev/moq/pull/5049), a fail-closed Python LLBC
emitter (Charon 0.1.284, nightly-2026-09-17). Its findings:

- moq-net's real `zigzag`/`unzigzag` translated to TypeScript on `u32`
  halves and matched native Rust over 15,900 cases. The generated integer
  entry point was 840 B gzip, on par with hand-written halves and faster
  than a BigInt control (Bun, a browser proxy). That is evidence to revisit
  the blanket ban on 64-bit bit operations once a codec slice passes.
- Hand-written probes covered nested options, clone/move/last-drop, early
  returns, unwind cleanup, and independent struct copies; general copy/drop
  lowering was not attempted.
- Charon also extracted `varint::size`; its enums/`Result`, tuples, calls,
  and halves casts were the next untranslated slice.

Do not post on rust-js without the maintainer's approval; it is outside our
orgs.

Public API: none. Wire: none.
