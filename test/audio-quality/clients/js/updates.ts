/**
 * Counts actual moq-lite SUBSCRIBE_UPDATE sends from the production viewer's debug log.
 * The optional publisher uses a real or explicitly fake camera through <moq-publish>.
 * This is a manual measurement, with no pass/fail rate budget.
 *
 * @module
 */
import { join, resolve } from "node:path";
import { parseArgs } from "node:util";
import { launch, sleep } from "../../../interop/clients/js/harness.ts";

const { values } = parseArgs({
	options: {
		page: { type: "string" },
		port: { type: "string", default: "0" },
		url: { type: "string" },
		broadcast: { type: "string" },
		camera: { type: "string" },
		duration: { type: "string", default: "60" },
		warmup: { type: "string", default: "5" },
		out: { type: "string" },
	},
});
const { url, broadcast, camera, out } = values;
const duration = Number(values.duration) * 1000;
const warmup = Number(values.warmup) * 1000;
if (
	!values.page ||
	!url ||
	!broadcast ||
	!out ||
	(camera !== undefined && camera !== "real" && camera !== "fake") ||
	!Number.isFinite(duration) ||
	duration <= 0 ||
	!Number.isFinite(warmup) ||
	warmup < 0
) {
	throw new Error(
		"usage: updates.ts --page DIR --url U --broadcast B --out FILE [--camera real|fake] [--duration S] [--warmup S]",
	);
}
const port = Number(values.port);
if (!Number.isInteger(port) || port < 0 || port > 65535) throw new Error("invalid page port");
const root = resolve(values.page);
const server = Bun.serve({
	port,
	hostname: "127.0.0.1",
	async fetch(req) {
		const file = Bun.file(join(root, new URL(req.url).pathname));
		return (await file.exists()) ? new Response(file) : new Response("not found", { status: 404 });
	},
});
const pageUrl = (file: string) => `http://127.0.0.1:${server.port}/${file}?${new URLSearchParams({ url, broadcast })}`;
// Grant autoplay explicitly: this measures subscription traffic, not the gesture gate.
const args = ["--autoplay-policy=no-user-gesture-required"];
if (camera === "fake") args.push("--use-fake-device-for-media-stream", "--use-fake-ui-for-media-stream");
let browser: Awaited<ReturnType<typeof launch>> | undefined;
try {
	browser = await launch(args);
	const context = await browser.newContext({ permissions: ["camera", "microphone"] });
	const notes: string[] = [];
	const failures: string[] = [];
	const events: { at: number; kind: string; id: string; track: string }[] = [];
	const origin = performance.now();
	if (camera) {
		const publisher = await context.newPage();
		publisher.on("pageerror", (error) => failures.push(`publisher: ${error.message}`));
		await publisher.goto(pageUrl("mic.html"));
		// The existing microphone recorder disables video; use its normal camera path too.
		await publisher.evaluate(() => document.querySelector("moq-publish")?.removeAttribute("invisible"));
		await publisher.waitForFunction(
			() => {
				const element = document.querySelector("moq-publish");
				return element?.audio.out.catalog.peek() && element.video.out.catalog.peek();
			},
			undefined,
			{ timeout: 30_000 },
		);
	}
	const page = await context.newPage();
	page.on("pageerror", (error) => failures.push(`viewer: ${error.message}`));
	page.on("console", (message) => {
		const text = message.text();
		// This log is emitted only after all chunks of the update have been written.
		const event = /^subscribe (start|update|close): id=(\d+) broadcast=.*? track=(.*)$/.exec(text);
		if (event) events.push({ at: performance.now() - origin, kind: event[1], id: event[2], track: event[3] });
		if (message.type() === "warning" || message.type() === "error") notes.push(text);
	});
	await page.goto(pageUrl("index.html"));
	await page.evaluate(() => document.querySelector("moq-watch")?.setAttribute("visible", "always"));
	await page.waitForFunction(
		() => {
			const element = document.querySelector("moq-watch");
			return (
				element?.audio.out.timestamp.peek() !== undefined && element?.video.out.timestamp.peek() !== undefined
			);
		},
		undefined,
		{ timeout: 30_000 },
	);
	const read = () =>
		page.evaluate(() => {
			const element = document.querySelector("moq-watch");
			if (!element) throw new Error("viewer disappeared");
			return {
				audio: element.audio.out.timestamp.peek(),
				video: element.video.out.timestamp.peek(),
				maxDelay: element.sync.out.maxDelay.peek(),
				transport: element.connection.transport.peek(),
				audioTrack: element.audio.source.out.track.peek(),
				videoTrack: element.video.source.out.track.peek(),
				error: String(
					element.error.peek() ??
						element.audio.out.error.peek() ??
						element.video.out.error.peek() ??
						element.connection.error.peek() ??
						"",
				),
			};
		});
	const ready = performance.now() - origin;
	const start = ready + warmup;
	const end = start + duration;
	const samples: { at: number; state: Awaited<ReturnType<typeof read>> }[] = [];
	while (performance.now() - origin < end) {
		samples.push({ at: performance.now() - origin, state: await read() });
		await sleep(Math.min(250, Math.max(0, end - (performance.now() - origin))));
	}
	samples.push({ at: performance.now() - origin, state: await read() });
	const first = samples.find((sample) => sample.at >= start)?.state;
	const last = samples.at(-1)?.state;
	if (!first || !last || first.audio === last.audio || first.video === last.video)
		failures.push("both media timestamps must advance in the measured window");
	for (const sample of samples) {
		if (sample.state.error) failures.push(sample.state.error);
		if (sample.state.transport !== "webtransport") failures.push(`transport: ${sample.state.transport}`);
	}
	for (const track of [first?.audioTrack, first?.videoTrack]) {
		if (!track || !events.some((event) => event.kind === "start" && event.track === track)) {
			failures.push(`missing moq-lite send log for selected media track: ${track}`);
		}
	}
	const tracks = [...new Set(events.map((event) => event.track))].map((track) => {
		const all = events.filter((event) => event.track === track);
		const steady = all.filter((event) => event.at >= start && event.at < end);
		const updates = steady.filter((event) => event.kind === "update");
		return {
			track,
			starts: all.filter((event) => event.kind === "start").length,
			warmupUpdates: all.filter((event) => event.kind === "update" && event.at < start).length,
			updates: updates.length,
			updatesPerSecond: updates.length / (duration / 1000),
		};
	});
	const result = {
		source: camera ? `${camera} camera and microphone` : "existing broadcast",
		url,
		broadcast,
		browser: browser.version(),
		warmup,
		duration,
		ready,
		start,
		end,
		tracks,
		failures: [...new Set(failures)],
		notes,
		samples,
		events,
	};
	await Bun.write(out, `${JSON.stringify(result, null, 2)}\n`);
	console.table(tracks);
	console.log(`${out}: ${result.source}; ${duration / 1000}s measured after ${warmup / 1000}s warmup`);
	if (result.failures.length) throw new Error(result.failures.join("\n"));
} finally {
	try {
		await browser?.close();
	} finally {
		server.stop(true);
	}
}
