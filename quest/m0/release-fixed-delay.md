# [XS] A fixed latency on release ignores the catalog jitter and delay

## Goal

The release deploy on `moq.dev` includes the fixed-delay backport. A viewer
who picks a fixed latency (100 ms, 250 ms, ...) plays at that delay even as
the rendition's catalog delay and jitter grow. Confirm the deployed revision
contains the fix and numeric selections no longer include those estimates.

## Plan

The implementation merged into release in
[moq-dev/moq#6217](https://github.com/moq-dev/moq/pull/6217). Verify its release
deploy reaches moq.dev, then retire this quest; no duplicate backport is
needed.

The backport preserves the behavior from
[moq-dev/moq#4162](https://github.com/moq-dev/moq/pull/4162) without its
breaking watch API rework: a numeric configured delay is used directly;
"auto" and "instant" remain unchanged. The release-side tests and live-browser
validation are recorded in the merged PR. Public API and wire are unchanged.

### Verification

Checked on 2026-10-23 UTC. The deployment condition is still unmet:

- [Release JS at the backport merge](https://github.com/moq-dev/moq/actions/runs/39810032442)
  succeeded, but its publish log says `@moq/watch@0.6.3 already published,
  skipping` at 2026-10-22T16:15:45Z. A green run does not establish that the
  changed package was published.
- The site's [lockfile at 8a7eec8](https://github.com/moq-dev/moq.dev/blob/8a7eec8e4be43127d33b4e88ad64a49cbb8fb4b6/bun.lock#L318)
  resolves `@moq/watch` to 0.6.2. This source pin does not identify the deployed
  build by itself.
- In a fresh Chrome tab, the [live demo](https://moq.dev/watch/?project=demo&name=bbb.hang)
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

Keep this quest until a package containing the backport is published and the
site deploy uses it. Then establish deployed build provenance and repeat the
numeric selections against nonzero, changing catalog delay/jitter, using the
resolved total-buffer diagnostic or controlled playback. The numeric target
must remain 100/250 ms as those estimates change. Release versioning and site
deployment remain outside this verification checkpoint; do not duplicate the
backport.
