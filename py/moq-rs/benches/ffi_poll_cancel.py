"""Spike for quest/m2/ffi-poll.md: cancellation semantics of each shape.

swallow: cancel a parked read at each tick after a frame lands, then read again. A
cancelled read that had already taken the frame swallows it.

shutdown: exit the process with a read parked; moq_ffi's atexit shutdown must not crash.
Prototype only; not for merge.
"""

import asyncio
import subprocess
import sys

import moq_ffi as ffi
from ffi_poll import READERS


def group_reader(shape):
    producer = ffi.MoqBroadcastProducer()
    track = producer.publish_track("t", None)
    group = track.append_group()
    consumer = group.consume()
    reader = consumer if shape == "task" else consumer.into_poll()
    if shape == "b1":
        from ffi_poll import registered

        registered(reader)
    return (producer, track), group, reader


async def swallow(shape: str, ticks: int) -> str:
    _, read_frame = READERS[shape][1:]
    keep, group, reader = group_reader(shape)
    first = asyncio.create_task(read_frame(reader))
    for _ in range(3):
        await asyncio.sleep(0)
    assert not first.done(), "the first read parks"
    group.write_frame(ffi.MoqFrame(payload=b"\x01", timestamp_us=None))
    for _ in range(ticks):
        await asyncio.sleep(0)
    first.cancel()
    try:
        frame = await first
        return f"first read won ({frame.payload[0]})"
    except asyncio.CancelledError:
        pass
    group.write_frame(ffi.MoqFrame(payload=b"\x02", timestamp_us=None))
    try:
        frame = await asyncio.wait_for(read_frame(reader), 2)
    except Exception as e:  # noqa: BLE001
        return f"second read failed: {type(e).__name__} {e}"
    return "ok" if frame.payload[0] == 1 else "SWALLOWED frame 1"


SHUTDOWN = """
import atexit, asyncio, sys, threading, time
sys.path.insert(0, {path!r})
state = {{}}

def wake_after_shutdown():
    # Runs after moq_ffi's own atexit shutdown (atexit is LIFO): the parked read polls again.
    state["group"].write_frame(state["frame"])
    time.sleep(0.2)
    print("woke after shutdown; read done =", state["done"](), file=sys.stderr)

atexit.register(wake_after_shutdown)

import moq_ffi as ffi
from ffi_poll_cancel import group_reader
from ffi_poll import READERS

keep, group, reader = group_reader({shape!r})
state["group"], state["frame"] = group, ffi.MoqFrame(payload=b"x", timestamp_us=None)
loop = asyncio.new_event_loop()
threading.Thread(target=loop.run_forever, daemon=True).start()
task = asyncio.run_coroutine_threadsafe(READERS[{shape!r}][2](reader), loop)
state["done"] = lambda: task.done() and repr(task.exception() or task.result().payload)
time.sleep(0.1)
assert not task.done()
"""


async def main() -> None:
    shapes = sys.argv[1:] or ["task", "a", "b", "b1"]
    for shape in shapes:
        outcomes = [await swallow(shape, ticks) for ticks in range(8)]
        print(f"swallow/{shape}: " + "; ".join(f"t{t}={o}" for t, o in enumerate(outcomes)))
    here = __import__("os").path.dirname(__file__)
    for shape in [s for s in shapes if s != "b1"]:
        proc = subprocess.run([sys.executable, "-c", SHUTDOWN.format(path=here, shape=shape)], capture_output=True, text=True)
        print(f"shutdown/{shape}: exit={proc.returncode} {proc.stderr.strip()[-300:]}")


if __name__ == "__main__":
    asyncio.run(main())
