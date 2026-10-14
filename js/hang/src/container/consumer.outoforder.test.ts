import { expect, jest, onTestFinished, spyOn, test } from "bun:test";
import { Group, Time, Track } from "@moq/net";
import { Signal } from "@moq/signals";
import { Consumer, type Read } from "./consumer.ts";
import { encodeFrame, Format } from "./legacy.ts";

// Drain the in-memory readers without advancing the mocked clock.
const flush = () => new Promise<void>((resolve) => setImmediate(resolve));

function setup(delay = 500, start?: number) {
	jest.useFakeTimers();
	const clock = spyOn(performance, "now").mockReturnValue(0);
	const track = new Track.Producer("test").accept({ timescale: Time.Timescale.MILLI, maxAge: Time.Milli(30_000) });
	const maxDelay = new Signal(Time.Milli(delay));
	const consumer = new Consumer(
		track.subscribe({
			maxDelay: Time.Milli(30_000),
			groups: start === undefined ? undefined : { start: { included: start } },
		}),
		{ format: new Format("data"), maxDelay },
	);
	const results: Read[] = [];
	const reading = (async () => {
		for (;;) {
			const next = await consumer.next();
			if (!next) return;
			results.push(next);
		}
	})();
	onTestFinished(async () => {
		consumer.close();
		track.close();
		try {
			await reading;
		} finally {
			clock.mockRestore();
			jest.useRealTimers();
		}
	});
	return { track, consumer, maxDelay, results };
}

function write(group: Group.Producer, millis: number) {
	group.writeFrame({
		payload: encodeFrame(new Uint8Array([1]), Time.Micro.fromMilli(Time.Milli(millis))),
		timestamp: Time.Timestamp.fromMillis(millis),
	});
}

function publish(track: Track.Producer, sequence: number, ...millis: number[]) {
	const group = new Group.Producer(sequence);
	track.writeGroup(group);
	for (const timestamp of millis) write(group, timestamp);
	group.close();
}

function items(results: Read[]) {
	return results.map((read) => (read.kind === "frame" ? `${read.group}@${read.frame.timestamp}` : read.kind));
}

test("an unfloored read settles the late window head at the exact media budget", async () => {
	const { track, results } = setup();
	publish(track, 3, 300);
	await flush();
	expect(results).toEqual([]);
	publish(track, 0, 0);
	publish(track, 1, 100);
	publish(track, 2, 200);
	await flush();
	expect(results).toEqual([]);
	publish(track, 4, 500);
	await flush();
	expect(items(results)).toEqual(["0@0", "1@100000", "2@200000", "3@300000", "4@500000"]);
});

test("a late window head keeps its tail after settlement", async () => {
	const { track, results } = setup(5000);
	const live = new Group.Producer(3);
	track.writeGroup(live);
	write(live, 3000);
	await flush();
	expect(results).toEqual([]);
	const head = new Group.Producer(1);
	track.writeGroup(head);
	write(head, 1000);
	write(live, 6000);
	await flush();
	expect(items(results)).toEqual(["1@1000000"]);
	write(head, 1500);
	head.close();
	publish(track, 2, 2000);
	await flush();
	expect(items(results)).toEqual(["1@1000000", "1@1500000", "2@2000000", "3@3000000", "3@6000000"]);
});

test("settled ordered reads discard backfill without a gap while arrivals retain it", async () => {
	const { track, maxDelay, results } = setup();
	const arrivals = track.subscribe({ maxDelay: Time.Milli(30_000) });
	onTestFinished(() => arrivals.close());
	publish(track, 3, 3000, 3500);
	(await arrivals.recvGroup())?.close();
	await flush();
	expect(items(results)).toEqual(["3@3000000", "3@3500000"]);
	// A wider budget cannot reopen history below the settled cursor.
	maxDelay.set(Time.Milli(5000));
	publish(track, 1, 1000);
	const backfill = await arrivals.recvGroup();
	expect(backfill?.sequence).toBe(1);
	backfill?.close();
	publish(track, 4, 3600);
	await flush();
	expect(items(results)).toEqual(["3@3000000", "3@3500000", "4@3600000"]);
});

test.each([false, true])("zero budget or declared start bypasses settlement (declared: %s)", async (declared) => {
	const { track, results } = setup(declared ? 500 : 0, declared ? 3 : undefined);
	publish(track, 3, 3000);
	await flush();
	expect(items(results)).toEqual(["3@3000000"]);
});

test("a declared window head can arrive after its successor", async () => {
	const { track, results } = setup(500, 3);
	publish(track, 4, 4000);
	await flush();
	expect(results).toEqual([]);
	publish(track, 3, 3500);
	await flush();
	expect(items(results)).toEqual(["3@3500000", "4@4000000"]);
});

test("a lower budget settles frames already buffered", async () => {
	const { track, maxDelay, results } = setup();
	publish(track, 3, 3000, 3300);
	await flush();
	expect(results).toEqual([]);
	maxDelay.set(Time.Milli(300));
	await flush();
	expect(items(results)).toEqual(["3@3000000", "3@3300000"]);
});

test("track completion settles a quiet unfloored read", async () => {
	const { track, results } = setup();
	publish(track, 3, 3000);
	await flush();
	expect(results).toEqual([]);
	track.close();
	await flush();
	expect(items(results)).toEqual(["3@3000000"]);
});
