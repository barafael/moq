# [M] A media hole in the player keeps the timeline

## Goal

When `<moq-watch>` skips a group (a max-delay skip or a hole timeout), audio
and video treat it as a hole on the same timeline: the decoder re-primes,
the audio ring keeps the PCM already decoded, and the shared `Sync` clock
keeps its anchor. Only a real new timeline (a declared end followed by a
later group, or a new instance) re-anchors, as Rust's `moq-audio` already
behaves.

## Plan

### Checkpoint: timeline preserved, video ownership unresolved

PR [moq-dev/moq#6903](https://github.com/moq-dev/moq/pull/6903) remains a draft.
The settled rule is unchanged: a hole re-primes codecs on the existing timeline,
keeping decoded PCM and Sync. A declared end followed by a later group, or a new
media instance, re-anchors. Existing `Container.Read` already distinguishes these;
no public API or wire change is needed.

Implemented and reproduced in both legacy and CMAF paths:

- Preserve PCM and Sync through holes, including parked decoded video.
- Preserve an audio endpoint across a hole and clip its own terminal group.
- Keep compatible PCM/Sync through same-instance codec/container changes. Genuine
  declared restarts/new instances still reset; explicit caller reset is unchanged.
- Clear discarded audio decode accounting through `ring.dropped()` at every codec
  reset. Six delayed 20 ms inputs otherwise fill the 100 ms floor after reset has
  discarded their output, blocking the resumed input forever.
- Sum disconnected received/decoded video spans consistently for startup and its
  decode gate, excluding the holes. Three 40 ms spans now stop decoding at six
  pictures while another track is pending, then release on that track's readiness.

The video ownership P1 is **not solved**. Legacy single-picture groups can have
unknown/zero duration. Their buffered ranges remain empty across holes, so no
amount of such received media satisfies the duration floor. The sum-only decode
gate therefore keeps decoding and parking GPU pictures indefinitely. A decoded
storage bound cannot depend only on that same unavailable duration.

### Decision needed

1. Separate decoded-storage ownership/backpressure from startup coverage, and
   choose an explicit startup policy. Recommended: preserve the current rule
   that missing media is not counted as received buffered media.
2. Change startup coverage semantics to count inferred holes as playable silence
   or held pictures. This changes the existing media-coverage rule.
3. Defer the larger design and retain this draft until a bounded design is chosen.

No storage budget, inferred duration, public shape, or coverage-policy change has
been selected while the maintainer is away. Keep this quest and PR draft until
both retained decoded state and startup progress are bounded by the chosen design.

### Portable zero-duration counterexample

In this PR checkout, append the following fixture case to
`js/watch/src/video/decoder.test.ts`, then run:

```bash
nix develop --command bun test js/watch/src/video/decoder.test.ts --test-name-pattern 'diagnostic:'
```

It uses that tracked file's existing `manualAt`, `guardedPlayback`, and
`legacyFrame` helpers, real MoQ Track/Container reads, and the configured max-delay
deadline. The failing assertion is intentional evidence, not retained CI coverage.
Close/reopen producer APIs should follow any upstream lifecycle migration when
replaying on a newer base.

```typescript
// BEGIN REAL SINGLE-FRAME DIAGNOSTIC
for (const count of [8, 16]) {
	it(`diagnostic: ${count} real single-picture groups retain startup output`, async () => {
		const clock = manualAt(0);
		const playback = await guardedPlayback("legacy", [], [], true, 1000);
		const remove = playback.sync.register(new Signal(Milli(100)), new Signal(null));
		if (!playback.track) throw new Error("missing video track");
		try {
			for (let n = 0; n < count; n++) {
				const group = new Moq.Group.Producer(n * 2);
				group.writeFrame(legacyFrame(Micro(n * 40_000)));
				group.close();
				playback.track.writeGroup(group);
				await microtasks();
			}
			await clock.advance(playback.sync.out.maxDelay.peek());
			await microtasks();
			console.log("real-single-picture-startup", JSON.stringify({input:count,submitted:playback.submitted.length,held:playback.pictures.filter(frame=>!frame.closed).length,playing:playback.sync.playing.peek(),buffered:playback.decoder.out.buffered.peek(),codecResets:playback.resets}));
			expect(playback.submitted.length).toBeLessThan(count);
		} finally {
			remove();playback.close();await microtasks();clock.installed.close();
		}
	});
}
// END REAL SINGLE-FRAME DIAGNOSTIC
```

Observed after the safe accounting/sum fixes, with a 100 ms floor and another
track pending:

```json
{"input":8,"submitted":8,"held":8,"playing":false,"buffered":[],"codecResets":7}
{"input":16,"submitted":16,"held":16,"playing":false,"buffered":[],"codecResets":15}
```

The first-to-last PTS spans are only 280/600 ms, within the configured 1100 ms
max delay. Each missing group expires under manual time. This is retained output
growth, not a timing/performance threshold. The fixture's pictures stand in for
native decoded GPU frames; it does not measure physical GPU allocation.

### Verification recorded so far

At `aeda750103e2dd4f6baa2c3599715bed5077337d`, `just fix`, clean-head `just ci`
(409 watch tests and affected dependents/API Extractor/tooling), and trusted
`quest check` passed. Chromium 156 native Opus/AudioWorklet proof kept the 40–120 ms
PCM span through an inferred 40 ms hole: codec resets 1, ring resets 0, Sync
resets 0, reference -100 ms; resumed PCM started at 140000 µs with nonzero tone
energy, and the parked Sync wait later presented. The dev player painted BBB.
The focused real Chromium media republish control passed at 29.9 fps over 118
presented frames with synchronized tone output; its children and ports were reaped.

The review-accounting regressions failed before the retained fixes and passed
after them (107 focused decoder tests). The final checkpoint's clean-head gates
and native surface result are recorded in the PR body before handoff. Passing
ordinary ranges and that browser proof do not solve the zero-duration ownership
counterexample above. No retry, sleep, larger timeout, deployment, or publication
is part of this checkpoint.

## Related

- [A/V clock](/quest/m1/av-clock.md) - the video-skip half, and the audio playhead driving `Sync`
- [Buffered playback subscribes in ascending order](/quest/m1/buffered-ascending.md) - the buffered-mode half of the same timeline
