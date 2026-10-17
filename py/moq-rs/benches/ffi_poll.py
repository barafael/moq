"""Spike for quest/m2/ffi-poll.md: today's Task reads against Shape A and Shape B.

Drives the generated moq_ffi bindings directly, so only the Rust-side shape and the
asyncio bridge differ between runs. Prototype only; not for merge.
"""

import asyncio
import json
import os
import resource
import sys
import time

import moq_ffi as ffi

BUDGET = float(os.environ.get("MOQ_BENCH_SECONDS", "1"))
SHAPES = os.environ.get("SPIKE_SHAPES", "task,a,b,b1").split(",")
SUBSCRIBERS = [int(s) for s in os.environ.get("SPIKE_SUBSCRIBERS", "1,4,16,64").split(",")]
SIZE = int(os.environ.get("SPIKE_SIZE", "1024"))
FRAMES = 64
CACHE_BYTES = 64 << 20
ROUND_TIMEOUT = 30


class CountingLoop(asyncio.SelectorEventLoop):
    """Counts thread-safe wakeups: each is a self-pipe write and a loop iteration."""

    wakeups = 0

    def call_soon_threadsafe(self, *args, **kwargs):
        CountingLoop.wakeups += 1
        return super().call_soon_threadsafe(*args, **kwargs)


def cpu() -> float:
    usage = resource.getrusage(resource.RUSAGE_SELF)
    return usage.ru_utime + usage.ru_stime


class FutureWaker(ffi.MoqWaker):
    """Resolves one asyncio future from any thread."""

    __slots__ = ("loop", "future")

    def __init__(self, loop: asyncio.AbstractEventLoop, future: asyncio.Future):
        self.loop = loop
        self.future = future

    def wake(self) -> None:
        self.loop.call_soon_threadsafe(_resolve, self.future)


def _resolve(future: asyncio.Future) -> None:
    if not future.done():
        future.set_result(None)


async def poll_ready(poll, ready_type):
    """Shape B's wrapper future: poll, and on Pending wait for the waker before polling again."""
    loop = asyncio.get_running_loop()
    while True:
        future = loop.create_future()
        result = poll(FutureWaker(loop, future))
        if isinstance(result, ready_type):
            return result
        await future


class RegisteredWaker(ffi.MoqWaker):
    """Shape B': registered once per handle; each wait swaps in a fresh future."""

    __slots__ = ("loop", "future")

    def __init__(self, loop: asyncio.AbstractEventLoop):
        self.loop = loop
        self.future = loop.create_future()

    def wake(self) -> None:
        self.loop.call_soon_threadsafe(_resolve, self.future)


async def poll_registered(handle, poll, ready_type):
    waker = handle._spike_waker
    while True:
        # Swap before polling, so a wake during the poll resolves the future we await.
        waker.future = waker.loop.create_future()
        result = poll()
        if isinstance(result, ready_type):
            return result
        await waker.future


def registered(handle):
    handle._spike_waker = RegisteredWaker(asyncio.get_running_loop())
    handle.set_waker(handle._spike_waker)
    return handle


async def b1_recv(track):
    group = (await poll_registered(track, track.poll_recv_group_registered, ffi.MoqPollGroupResult.READY)).group
    return group and registered(group)


async def b1_read(group):
    return (await poll_registered(group, group.poll_read_frame_registered, ffi.MoqPollFrameResult.READY)).frame


# Each shape: how to turn a fresh track consumer into a reader, and its two hot calls.
async def task_recv(track):
    return await track.recv_group()


async def task_read(group):
    return await group.read_frame()


async def a_recv(track):
    return await track.recv_group()


async def a_read(group):
    return await group.read_frame()


async def b_recv(track):
    return (await poll_ready(track.poll_recv_group, ffi.MoqPollGroupResult.READY)).group


async def b_read(group):
    return (await poll_ready(group.poll_read_frame, ffi.MoqPollFrameResult.READY)).frame


READERS = {
    "task": (lambda consumer: consumer, task_recv, task_read),
    "a": (lambda consumer: consumer.into_poll(), a_recv, a_read),
    "b": (lambda consumer: consumer.into_poll(), b_recv, b_read),
    "b1": (lambda consumer: registered(consumer.into_poll()), b1_recv, b1_read),
}


async def read(shape, track, payload, n):
    _, recv, read_frame = READERS[shape]
    group = await recv(track)
    assert group is not None, "track ended"
    for f in range(FRAMES):
        frame = await read_frame(group)
        assert frame is not None, "group ended"
        assert frame.payload == payload, f"corrupt frame {f} of round {n}"
        assert frame.timestamp_us == n * FRAMES + f


async def throughput(shape: str, subscribers: int, live: bool, results: list) -> None:
    wrap, _, _ = READERS[shape]
    origin = ffi.MoqOriginProducer(ffi.MoqOriginConfig(cache_capacity_bytes=CACHE_BYTES))
    broadcast = origin.create_broadcast("bench")
    info = ffi.MoqTrackInfo(priority=127, max_age_us=None, timescale=1_000_000)
    track = broadcast.publish_track("0", info)
    readers = [wrap(track.consume(None)) for _ in range(subscribers)]
    payload = bytes(i % 251 for i in range(SIZE))

    async def step(n: int) -> None:
        tasks = [asyncio.create_task(read(shape, r, payload, n)) for r in readers]
        if live:
            # Let every reader park first, then deliver each frame to parked readers.
            await asyncio.sleep(0)
        group = track.append_group()
        for f in range(FRAMES):
            group.write_frame(ffi.MoqFrame(payload=payload, timestamp_us=n * FRAMES + f))
            if live:
                for _ in range(3):
                    await asyncio.sleep(0)
        group.finish()
        await asyncio.wait_for(asyncio.gather(*tasks), ROUND_TIMEOUT)

    for n in range(3):
        await step(n)
    ffi.spike_counters()
    CountingLoop.wakeups = 0
    start, cpu_start, n = time.perf_counter(), cpu(), 3
    while time.perf_counter() - start < BUDGET:
        await step(n)
        n += 1
    wall, cpu_used = time.perf_counter() - start, cpu() - cpu_start
    counters = ffi.spike_counters()
    rounds = n - 3
    frames = rounds * subscribers * FRAMES
    calls = rounds * subscribers * (FRAMES + 1)
    mb = frames * SIZE / 1e6
    name = f"{shape}/{'live' if live else 'burst'}/size={SIZE}/sub={subscribers}"
    results.append({"name": f"throughput/{name}", "unit": "MB/s", "value": mb / wall})
    results.append({"name": f"cpu/{name}", "unit": "us/frame", "value": cpu_used * 1e6 / frames})
    results.append({"name": f"polls/{name}", "unit": "per call", "value": counters.polls / calls})
    results.append({"name": f"wakes/{name}", "unit": "per call", "value": counters.wakes / calls})
    results.append({"name": f"loop-wakeups/{name}", "unit": "per frame", "value": CountingLoop.wakeups / frames})
    for r in readers:
        r.cancel()


async def latency(shape: str, results: list) -> None:
    """Per-call cost of a read whose frame is already there: no wait, only the call."""
    producer = ffi.MoqBroadcastProducer()
    track = producer.publish_track("calls", None)
    count, groups, elapsed = 1000, 10, 0.0
    for _ in range(groups):
        group = track.append_group()
        for _ in range(count):
            group.write_frame(ffi.MoqFrame(payload=b"x", timestamp_us=None))
        group.finish()
        consumer = group.consume()
        reader = consumer if shape == "task" else consumer.into_poll()
        if shape == "b1":
            registered(reader)
        read_frame = READERS[shape][2]
        start = time.perf_counter()
        for _ in range(count):
            await read_frame(reader)
        elapsed += time.perf_counter() - start
    results.append({"name": f"call/{shape}/read_frame_ready", "unit": "ns", "value": elapsed * 1e9 / (count * groups)})


async def main() -> None:
    results: list = []
    # Interleave shapes inside each sweep point, so host load drifts across all of them alike.
    for _ in range(int(os.environ.get("SPIKE_REPEATS", "1"))):
        for shape in SHAPES:
            await latency(shape, results)
        for live in (False, True):
            for subscribers in SUBSCRIBERS:
                for shape in SHAPES:
                    await throughput(shape, subscribers, live, results)
    with open(os.environ.get("SPIKE_OUT", "/dev/stdout"), "w") as out:
        json.dump(results, out, indent=1)


if __name__ == "__main__":
    asyncio.run(main(), loop_factory=CountingLoop)
