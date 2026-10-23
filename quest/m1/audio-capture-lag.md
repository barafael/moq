# [S] Camera audio stops falling behind video

## Goal

A physical camera and microphone published from `js/publish` keep audio
timestamps beside video's, with no growing backlog. The audio catalog `delay`
settles, and a viewer sends no steady stream of `subscribe update: id=` lines.

## Plan

On 2026-10-23 a physical camera and microphone published from `moq.dev/publish`
(release) fell further behind the wall clock at about 0.25 s per second,
unbounded. Each rise republished the catalog, and the viewer pushed a new
`maxDelay` to every subscription: 146 video and 176 audio SUBSCRIBE_UPDATEs in
60 seconds. A 25% shortfall is far beyond device clock drift (parts per
million), and the measurement could not tell which of two causes it was:

- **Lost samples.** `js/publish/src/audio/capture-worklet.ts` stamps each quantum
  as `sampleCount / sampleRate` past one wall-clock anchor. A quantum with no
  input channels returns before counting, and a quantum the render thread drops
  is never counted, so the timestamps fall behind real time.
- **Encoder backlog.** The timestamps are right, but the Opus encoder runs slower
  than real time, so frames leave later and later and the queue grows without
  bound.

Both read as a rising audio `delay` from timestamp order, but they need
different fixes. First tell them apart on a physical camera: log, once a
second, the worklet's latest stamp minus `now()` and the `AudioEncoder`'s
`encodeQueueSize`. A growing stamp gap is lost samples; a growing queue is the
backlog.

For lost samples, keep the stamp on real time without resampling the drift into
the audio, such as anchoring on the context's clock (`currentTime` through
`getOutputTimestamp()`) or stamping a measured gap the framer already re-anchors
on. For a backlog, find why the encoder falls behind and shed load before the
queue grows, rather than letting latency climb.
