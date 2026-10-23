# [XS] A fixed latency on release ignores the catalog jitter and delay

## Goal

The release deploy on `moq.dev` includes the fixed-delay backport. A viewer
who picks a fixed latency (100 ms, 250 ms, ...) plays at that delay even as
the rendition's catalog delay and jitter grow. Confirm the deployed revision
contains the fix and numeric selections no longer include those estimates.

## Plan

The implementation merged into release in
[moq-dev/moq#6217](https://github.com/moq-dev/moq/pull/6217). Verify its release
deploy reaches moq.dev, then retire this quest. Published `@moq/watch@0.6.5`
contains the fix; the remaining condition is the site's dependency,
deployment provenance, and live verification. No duplicate backport is needed.

The backport preserves the behavior from
[moq-dev/moq#4162](https://github.com/moq-dev/moq/pull/4162) without its
breaking watch API rework: a numeric configured delay is used directly;
"auto" and "instant" remain unchanged. The release-side tests and live-browser
validation are recorded in the merged PR. Public API and wire are unchanged.

### Verification

Checked on 2026-10-23 UTC. Package publication is proven; the site's deployment
condition remains unverified:

- The [published 0.6.5 package metadata](https://registry.npmjs.org/@moq/watch/0.6.5)
  identifies commit
  [4f401ab](https://github.com/moq-dev/moq/commit/4f401ab94e00ae97134b8ad608e08d371c662ab4).
  Its [published archive](https://registry.npmjs.org/@moq/watch/-/watch-0.6.5.tgz)
  contains `player-DVWG4_Zd.js`: its numeric branch sets the resolved delay
  directly and returns before adding media delay and jitter. The accompanying
  `player-DVWG4_Zd.js.map` embeds `../src/sync.ts`; its `#runDelay` contains the
  exact numeric early return from the backport. This verifies changed package
  bytes, independently of the earlier successful Release JS run that skipped
  already-published 0.6.3.
- The site's current main remains
  [8a7eec8](https://github.com/moq-dev/moq.dev/commit/8a7eec8e4be43127d33b4e88ad64a49cbb8fb4b6).
  Its [lockfile](https://github.com/moq-dev/moq.dev/blob/8a7eec8e4be43127d33b4e88ad64a49cbb8fb4b6/bun.lock#L318)
  still resolves `@moq/watch` to 0.6.2. This source pin does not identify the
  deployed build by itself, and no deployed revision was established.
- In the earlier fresh Chrome tab, the [live demo](https://moq.dev/watch/?project=demo&name=bbb.hang)
  played video. Settings > Latency showed an active 100 ms preset with
  **Jitter buffer 100 ms / Total buffer 475 ms**, then an active 250 ms preset
  with **250 ms / 625 ms**. Returning to 100 ms repeated **100 ms / 475 ms**.
  These are the player's displayed buffer diagnostics, not a glass-to-glass
  measurement or proof that the extra 375 ms comes from particular catalog
  fields.
- The browser observed `/_astro/watch.DDINuY1C.js` and
  `/_astro/element.gftmzwNG.js`. No deployed revision was exposed, and the
  browser's asset export does not support scripts, so the deployed fixed-delay
  branch was not verified from these asset names.

Keep this quest until the site deploy uses the corrected package. Establish
deployed build provenance and repeat the
numeric selections against nonzero, changing catalog delay/jitter, using the
resolved total-buffer diagnostic or controlled playback. The numeric target
must remain 100/250 ms as those estimates change. Release versioning and site
deployment remain outside this verification checkpoint; do not duplicate the
backport.
