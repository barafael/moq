# [L] Generated JS API reference

## Goal

The doc site has an API reference for every published `@moq/*` package,
including `@moq/watch`, `@moq/publish`, `@moq/libav`, and `@moq/boy`,
which JSR cannot host because of their global type augmentation. Each package
page under `doc/lib/js/` links its reference.

## Plan

`doc/.vitepress/api/` renders the reference with TypeDoc at doc build time:
one page per entrypoint under `/lib/js/api/`, an anchor per symbol, and a
TypeDoc warning fails the build. Each package guide links its page. Fixtures in
`doc/.vitepress/api/index.test.ts` cover export names, cross-package
re-exports, hidden declarations, and anchors.

One decision remains: whether TypeDoc also replaces API Extractor for the
committed `js/*/api/*.api.md` reports, in this PR, in a follow-up quest, or
never. Once it is settled, this quest is done.

Decided 2026-10-23 (maintainer interview), amended 2026-10-27:

- TypeDoc (with typedoc-plugin-markdown and typedoc-vitepress-theme) renders
  from source, replacing the earlier plan of API Extractor's doc model plus
  our own renderer. That model lost public export names (`Error_2`) and
  dropped external re-exports. TypeScript 6 is pinned for TypeDoc alone until
  it supports TypeScript 7.
- One page per entrypoint, with an anchor per symbol: the same split as the
  `.api.md` reports.
- Generated at doc build time from the checkout being built, so it always
  matches the guides beside it. Nothing generated is committed.
- JSR keeps publishing its own reference; this does not replace it.
- Rust is a non-goal: docs.rs covers it.
