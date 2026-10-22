# [S] A viewer's subscription updates stop chattering

## Goal

A `js/watch` viewer sends a SUBSCRIBE_UPDATE only when its subscription
meaningfully changes, not several times a second per track. In a reported
`moq.dev/publish` session, the publisher logged an endless alternating
`subscribe update` for audio and video.

## Plan

The manual probe in [moq-dev/moq#6305](https://github.com/moq-dev/moq/pull/6305)
measured current `js/watch` and `js/publish` at `5d5974f083` using the Nix-pinned
Chromium 156.0.8078.4 on Linux. Both rows ran for 60 seconds after five seconds
of warmup from first audio/video, over WebTransport through `cdn.moq.dev`:

| Source | Track | Initial subscriptions | Warmup updates | Measured updates |
| --- | --- | --- | --- | --- |
| `demo/bbb.hang` | `2.aac` | 1 | 0 | 0 |
| `demo/bbb.hang` | `1.avc3` | 1 | 1 | 0 |
| generated camera/microphone through `anon` | `audio` | 1 | 0 | 0 |
| generated camera/microphone through `anon` | `video` | 1 | 6 | 0 |

Audio/video timestamps advanced in both rows, and catalog updates were zero.
The generated devices are Chromium's fake capture control, not physical-camera
evidence. This host has no `/dev/video*` device. These rows do not reproduce the
reported steady chatter, so no rounding or rate limit has been selected.

Remaining: run `just test subscribe-updates --camera real` with a fresh broadcast
name on a camera-equipped host (the full command is in the audio-quality README).
Then decide whether the original live-camera report still needs a behavior change,
or whether this quest is unneeded. Keep the current subscription behavior until
that measurement supports a cut. The probe preserves per-track sends and max-delay
samples without adding subscriptions or a traffic-rate budget.

Cause, from reading the code on 2026-10-20: `js/watch/src/media.ts` pushes
`maxDelay` into `subscriber.update` on every change, and the subscriber loop
(`js/net/src/lite/subscriber.ts`, `#sameSubscription`) compares it exactly.
Video's `maxDelay` follows `sync.out.maxDelay`, which moves with the per-frame
arrival spread (20 ms steps, at most every 500 ms) and every catalog estimate
republish. Audio only joins once `maxDelay` passes `AUTO_MAX_DELAY`.

Settled: measure first with the bounded publisher encode queue. Count updates
per track for a live camera publish and `bbb.hang`, then pick a cut with the numbers, and
close this as unneeded if they are already quiet. Options weighed when
planning:

- Round the subscription's `maxDelay` up to the next power of two ms in
  `js/watch`, so it changes only when the target crosses a doubling.
- Rate-limit sends to one per smoothed RTT per track in `js/net` and
  `rs/moq-net`, latest value wins.

Coalescing to one update "in flight" was ruled out: moq-lite's
SUBSCRIBE_UPDATE has no acknowledgement, and the sender already sends only the
latest value whenever the stream applies backpressure.
