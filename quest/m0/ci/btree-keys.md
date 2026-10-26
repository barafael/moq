# [S] Fewer BTreeMap instantiations in moq-net

## Goal

moq-net instantiates BTreeMap/BTreeSet for fewer key and value types, which make
up 22.7% of its library's 1,360,948 LLVM IR lines at main 65ea670311 (the
[m2 compile-time audit](/quest/m2/audit-compile-time.md) counts 29.5% with
different features on an earlier tree). Record the library's and test target's CPU time and `cargo
llvm-lines` before and after, and keep the change only if the CPU time drops
beyond noise.

## Plan

Decided with the maintainer on 2026-10-25, as the second "thin generics" rule,
measured before adoption.

- List the instantiations by key and value type (`cargo llvm-lines` groups them).
- Options per site:
  - share one key type (for example a `u64` sequence wrapper);
  - use a `Vec` kept sorted where it stays small;
  - erase the value behind an index into a slab.
- Stay on std's map. A spike on 2026-10-25 found bbtree 0.1.5 slower on
  `TrackState`'s ascending insert, front pruning, and range lookups (2-4x on
  insert), about 9x std's IR per instantiation, and a hard dependency on Verus's
  `vstd`. Its concurrent map won only on contended reads of maps over 10k
  entries, and its scans can't give `containing()` one consistent view. Revisit
  it once it leaves 0.1.x with `vstd` optional.
- Keep each change local, and don't trade a lookup's complexity for build time on
  a hot path without a benchmark.

Public API: none.

The [2026-10-26 experiment](/.github/btree-build-audit.md) inventories 38 concrete
map key/value spellings and preserves an exact candidate patch plus eight raw
samples. Sharing the expiry wall index's lossless u64 nanosecond key removes
6,485 LLVM lines from both library and unit-test targets, but CPU ranges overlap
and their median directions disagree on the shared host. The candidate was
removed after measurement, so no production representation change is retained.

Remaining decision: repeat that patch on a quiet host (recommended), try a
larger local candidate with its own runtime benchmarks, or deprioritize the
build-cost work. Do not infer a speedup from the IR count alone. Retain a
candidate only after the CPU evidence distinguishes it from noise, then run
its semantic/runtime checks and the selected local gates.
