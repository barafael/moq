# [L] Generated JS API reference

## Goal

The doc site has an API reference for every published `@moq/*` package,
including `@moq/watch`, `@moq/publish`, `@moq/libav`, and `@moq/boy`,
which JSR cannot host because of their global type augmentation. Each package
page under `doc/lib/js/` links its reference.

## Plan

The shared extractor can now emit doc models with its existing visibility checks.
`doc/.vitepress/api-models.ts` generates 35 entrypoint models for all 18
published packages from isolated declaration snapshots, without touching their
normal `dist/` outputs. The renderer remains unwired and the ordinary doc build
is unchanged. `doc/AGENTS.md` is unchanged until the reference exists.

### Input fidelity needs a decision

API Extractor 7.59.4's models lose public export identity. The canonical
reference, export navigation (`isExported`), excerpt and source metadata do
not recover it. Two minimal counterexample pairs run in
`js/common/api.test.ts`:

- `export * as Error from "./errors"`, where `errors.d.ts` exports
  `function fail(): Error`, emits the namespace `Error_2`, canonical reference
  `@moq/example!Error_2:namespace`, with an empty excerpt and source
  `index.d.ts`. Changing the actual public export to `Error_2` produces an
  identical complete model. Stripping `_2` would therefore guess incorrectly
  for one input.
- `export { Thing as First } from "dep"` and `export { Thing as Second }
  from "dep"` produce identical models with no entrypoint members. The
  dependency declares `export class Thing {}`. The extractor omits these
  external re-export aliases entirely.

Real packages show the same problem: `@moq/net` exports `Error` and `Origin`,
but their model names are `Error_2` and `Origin_2`; `@moq/auth`'s `Request`
and `@moq/json`'s `Window` become `Request_2` and `Window_2`.
`@moq/net`'s `Path.Pattern` re-export is absent. The hidden/exported same-name
and alias-chain visibility fixtures are also retained and pass.

Neither option below is selected:

1. **Recommended:** admit an explicit symbol export/alias manifest from the
   existing compiler/extraction step, in or beside the models. Models remain
   authoritative for declarations and TSDoc; the manifest preserves public
   names and external re-export targets. This amends the model-only input rule.
2. Keep the model-only rule. Defer a complete reference until the extractor
   preserves that information, or explicitly narrow the Goal to the exports
   its models faithfully represent. Emitted helper names and omitted public
   re-exports cannot silently count as the complete reference.

After that decision, finish the renderer, actual export-name and cross-package
link fixtures, entrypoint pages/anchors, package-guide links, site/browser
proof, and the requested factual `doc/AGENTS.md` correction. No alias guessing
or narrowed coverage has been adopted.

Decided 2026-10-23 (maintainer interview):

- API Extractor's doc model (`.api.json`), from the shared per-package config
  in `js/common/api.ts`, is the only source. That config turns
  `ae-unresolved-link` off, so links the reference renders may need fixing.
  The reference is TSDoc next to the code, rendered; update the
  "not the reference" line in `doc/AGENTS.md` to say so.
- Our own renderer, not api-documenter: a small script turns the doc model
  into VitePress pages styled like the rest of the site.
- One page per entrypoint (such as `/lib/js/net/api`), with an anchor per
  symbol: the same split as the `.api.md` reports.
- Generated at doc build time from the checkout being built, so it always
  matches the guides beside it. Nothing generated is committed.
- JSR keeps publishing its own reference; this does not replace it.
- Rust is a non-goal: docs.rs covers it.
- The link resolver (`nameableReference` from
  [moq-dev/moq#6794](https://github.com/moq-dev/moq/pull/6794)) gets fixtures
  for a hidden and an exported declaration sharing a name, and for chains of
  aliases.
