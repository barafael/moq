# [S] Audio capture follows the shared clock

## Goal

`js/publish` stamps captured audio on the clock video uses, so a physical
microphone's timestamps stay beside its camera's instead of falling behind them.
A `moq.dev/publish` camera and microphone advertise a steady audio catalog
`delay`, and a viewer sends no steady stream of `subscribe update: id=` lines.

## Plan

`js/publish/src/audio/capture-worklet.ts` stamps each quantum as
`sampleCount / sampleRate` past a wall-clock anchor taken once at creation.
A capture device that delivers fewer samples per second than its nominal rate
(or a render thread that drops quanta) loses time against the wall clock that
video frames are stamped on.

On 2026-10-23 a physical camera and microphone published from `moq.dev/publish`
(release) drifted audio about 0.25 s per second behind video, unbounded.
Each rise republished the catalog, and the viewer pushed a new `maxDelay` to
every subscription: 146 video and 176 audio SUBSCRIBE_UPDATEs in 60 seconds.
The catalog `delay` now comes from timestamp order across renditions, so the
drift still reads as a rising audio `delay`; it is real, and only fixing the
clock removes it.

Reproduce first and confirm the clock is the cause (log the worklet's stamp
minus `now()` per second). Then tie the stamp to the context's own clock, such
as `currentTime` mapped through `getOutputTimestamp()`, or re-anchor on a
measured gap the way the framer already does, without resampling drift into
the audio itself.
