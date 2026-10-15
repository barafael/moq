import { expect, test } from "bun:test";
import { announcedView } from "./announced.test-util.ts";
import { accept, connect } from "./connection/index.ts";
import * as Ietf from "./ietf/index.ts";
import * as Lite from "./lite/index.ts";
import { createMockTransportPair } from "./mock.ts";
import { Producer } from "./origin.ts";
import * as Path from "./path.ts";
import { wireOf } from "./wire.ts";

const scope = (include: string[], exclude: string[]) =>
	new Path.Scope({
		include: new Path.Patterns(include.map(Path.Pattern.parse)),
		exclude: new Path.Patterns(exclude.map(Path.Pattern.parse)),
	});

test("rooted exclusions hide reserved paths and retain private names and boundaries", async () => {
	const origin = new Producer();
	const broadcasts = ["cam", ".private/cam", ".reserved/cam", ".reserved-other/cam"].map((path) => {
		const broadcast = origin.createBroadcast(Path.from(`tenant/${path}`));
		broadcast.announce();
		return broadcast;
	});
	const view = origin.scope(Path.from("tenant"), scope(["**"], [".reserved/**"]));
	const consumer = view.consume();
	const announces = await announcedView(consumer, undefined, { hidden: true });
	try {
		expect([...announces.peek().keys()].sort()).toEqual(
			[".private/cam", ".reserved-other/cam", "cam"].map((path) => Path.from(path)),
		);
		expect(() => consumer.request(Path.from(".reserved/cam"))).toThrow("outside the origin scope");
		expect(() => view.scope(Path.from(".reserved"), new Path.Patterns([Path.Pattern.all()]))).toThrow(
			"do not overlap",
		);
		expect(view.allowed().matches(".private/cam")).toBe(true);
		expect(view.allowed().matches(".reserved/cam")).toBe(false);
	} finally {
		await announces.close();
		for (const broadcast of broadcasts) broadcast.close();
		origin.close();
	}
});

test("an excluded producer cannot publish, advertise, or escape through a broad route", () => {
	const origin = new Producer();
	const view = origin.scope(Path.empty(), scope(["**"], ["reserved/**"]));
	expect(() => view.createBroadcast(Path.from("reserved/cam"))).toThrow("outside the origin scope");
	expect(() => view.dynamic(Path.from("reserved"))).toThrow("outside the origin scope");
	const broad = view.dynamic(Path.empty());
	expect(wireOf(origin.consume()).routes(Path.from("reserved/cam"))).toBe(false);
	expect(wireOf(origin.consume()).routes(Path.from("cam"))).toBe(true);
	expect(() => origin.scope(Path.empty(), scope(["room/**"], ["room", "room/*", "room/*/*/**"]))).toThrow(
		"do not overlap",
	);
	broad.close();
	origin.close();
});

for (const protocol of [Lite.ALPN_05, Lite.ALPN_07_WIP, Ietf.ALPN.DRAFT_16, Ietf.ALPN.DRAFT_17, Ietf.ALPN.DRAFT_22]) {
	test(`${protocol}: an excluded view keeps media working with an unknown credential grant`, async () => {
		const origin = new Producer();
		const broadcast = origin.createBroadcast(Path.from("tenant/cam"));
		broadcast.announce();
		broadcast
			.createTrack("video")
			.appendGroup()
			.writeFrame({ payload: new Uint8Array([42]) });
		const view = origin.scope(Path.from("tenant"), scope(["**"], ["reserved/**"]));
		const transport = createMockTransportPair(protocol);
		const url = new URL("https://localhost/scope");
		const [client, server] = await Promise.all([
			connect({ url, transport: transport.client, publish: view.consume() }),
			accept({ url, transport: transport.server }),
		]);
		const track = wireOf(server).consume(Path.from("cam")).track("video").subscribe().ordered();
		try {
			const group = await track.nextGroup();
			expect(group).toBeDefined();
			expect((await group?.readFrame())?.payload).toEqual(new Uint8Array([42]));
			expect(server.auth.grant.peek()).toBeUndefined();
		} finally {
			track.close();
			client.abort();
			server.abort();
			broadcast.close();
			origin.close();
			await Promise.all([client.closed, server.closed]);
		}
	});
}
