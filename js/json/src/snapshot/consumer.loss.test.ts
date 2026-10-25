import { expect, test } from "bun:test";
import { Error as NetError, StreamCode, Track } from "@moq/net";
import { Consumer } from "./consumer.ts";

const frame = (json: string) => ({
	payload: new TextEncoder().encode(json),
});

for (const method of ["next", "latest"] as const) {
	test(`snapshot ${method} resumes after an established group is missing`, async () => {
		const track = new Track.Producer("catalog.json");
		const consumer = new Consumer<{ value: number }>({ track: track.subscribe() });
		const first = track.appendGroup();
		first.writeFrame(frame('{"value":1}'));
		expect((await consumer[method]())?.value).toEqual({ value: 1 });

		const pending = consumer[method]();
		// Before lite-07, SUBSCRIBE_DROP reports a missing group as NotFound.
		first.close(new NetError.Stream(StreamCode.NotFound, { message: "dropped" }));
		const replacement = track.appendGroup();
		replacement.writeFrame(frame('{"value":2}'));
		replacement.close();
		track.close();

		expect((await pending)?.value).toEqual({ value: 2 });
	});

	for (const established of [false, true]) {
		test(`snapshot ${method} reports terminal track NotFound (${established ? "established" : "missing"})`, async () => {
			const track = new Track.Producer("catalog.json");
			const consumer = new Consumer<{ value: number }>({ track: track.subscribe() });
			if (established) {
				track.appendGroup().writeFrame(frame('{"value":1}'));
				expect((await consumer[method]())?.value).toEqual({ value: 1 });
			}

			const pending = consumer[method]();
			const failure = new NetError.Stream(StreamCode.NotFound);
			track.close(failure);
			await expect(pending).rejects.toBe(failure);
		});
	}

	test(`snapshot ${method} refuses malformed JSON after a good value`, async () => {
		const track = new Track.Producer("catalog.json");
		const consumer = new Consumer<{ value: number }>({ track: track.subscribe() });
		const group = track.appendGroup();
		group.writeFrame(frame('{"value":1}'));
		expect((await consumer[method]())?.value).toEqual({ value: 1 });

		group.writeFrame(frame('{"value":'));
		group.close();
		track.close();
		await expect(consumer[method]()).rejects.toBeInstanceOf(SyntaxError);
	});
}
