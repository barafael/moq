# `@moq/hang`

```ts
export * as Catalog from "@moq/hang/catalog";

export * as Container from "@moq/hang/container";

export namespace Echo {
	export type Arrival = z.infer<typeof ArrivalSchema>;
	export const ArrivalSchema: z.ZodMiniObject<{
		timestamp: z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">;
		wall: z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">;
	}, z.core.$strip>;
	export type Rendition = z.infer<typeof RenditionSchema>;
	export const RenditionSchema: z.ZodMiniObject<{
		arrival: z.ZodMiniOptional<z.ZodMiniPipe<z.ZodMiniNullable<z.ZodMiniObject<{
			timestamp: z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">;
			wall: z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">;
		}, z.core.$strip>>, z.ZodMiniTransform<{
			timestamp: number & z.$brand<"u53">;
			wall: number & z.$brand<"u53">;
		} | undefined, {
			timestamp: number & z.$brand<"u53">;
			wall: number & z.$brand<"u53">;
		} | null>>>;
		bytes: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>;
		decoded: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>;
		errors: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>;
		frames: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>;
		late: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>;
		latency: z.ZodMiniOptional<z.ZodMiniPipe<z.ZodMiniNullable<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>, z.ZodMiniTransform<number & z.$brand<"u53"> | undefined, number & z.$brand<"u53"> | null>>>;
		stalled: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>;
		stalls: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>;
		underruns: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>;
	}, z.core.$strip>;
	export const SUFFIX: ".echo";
	export type Snapshot = z.infer<typeof SnapshotSchema>;
	export const SnapshotSchema: z.ZodMiniObject<{
		renditions: z.ZodMiniOptional<z.ZodMiniRecord<z.ZodMiniString<string>, z.ZodMiniObject<{
			arrival: z.ZodMiniOptional<z.ZodMiniPipe<z.ZodMiniNullable<z.ZodMiniObject<{
				timestamp: ...;
				wall: ...;
			}, z.core.$strip>>, z.ZodMiniTransform<{
				timestamp: ...;
				wall: ...;
			} | undefined, {
				timestamp: ...;
				wall: ...;
			} | null>>>;
			bytes: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>;
			decoded: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>;
			errors: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>;
			frames: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>;
			late: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>;
			latency: z.ZodMiniOptional<z.ZodMiniPipe<z.ZodMiniNullable<z.core.$ZodBranded<z.ZodMiniNumber<...>, "u53", "out">>, z.ZodMiniTransform<... | undefined, ... | null>>>;
			stalled: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>;
			stalls: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>;
			underruns: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>;
		}, z.core.$strip>>>;
		transport: z.ZodMiniOptional<z.ZodMiniObject<{
			age: z.ZodMiniOptional<z.ZodMiniPipe<z.ZodMiniNullable<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>, z.ZodMiniTransform<... & ... | undefined, ... & ... | null>>>;
			lostBytes: z.ZodMiniOptional<z.ZodMiniPipe<z.ZodMiniNullable<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>, z.ZodMiniTransform<... & ... | undefined, ... & ... | null>>>;
			lostPackets: z.ZodMiniOptional<z.ZodMiniPipe<z.ZodMiniNullable<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>, z.ZodMiniTransform<... & ... | undefined, ... & ... | null>>>;
			rate: z.ZodMiniOptional<z.ZodMiniPipe<z.ZodMiniNullable<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>, z.ZodMiniTransform<... & ... | undefined, ... & ... | null>>>;
			rtt: z.ZodMiniOptional<z.ZodMiniPipe<z.ZodMiniNullable<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>, z.ZodMiniTransform<... & ... | undefined, ... & ... | null>>>;
		}, z.core.$strip>>;
	}, z.core.$loose>;
	export const TRACK: "feedback.json";
	export function isEcho(path: Valid): boolean;
}

export namespace Health {
	export interface Connection {
		evidence: Evidence[];
		lostBytes?: number;
		lostPackets?: number;
		stale?: true;
		verdict: Verdict;
	}
	export interface Evidence {
		input: Input;
		value?: number;
		verdict: Verdict;
	}
	export interface Health {
		interval?: number;
		observer: Observer;
		renditions: Record<string, Rendition>;
		reset?: Reset;
		role: Role;
		stale?: true;
		transport: Connection;
		verdict: Verdict;
	}
	export type Input = "rtt" | "loss" | "headroom" | "skipped" | "late" | "errors" | "stalled";
	export interface Limits {
		degraded: number;
		unhealthy: number;
	}
	export interface Observer {
		epoch?: Valid;
		path: Valid;
	}
	export interface Rendition {
		bitrate?: number;
		bytes?: number;
		evidence: Evidence[];
		frames?: number;
		verdict: Verdict;
	}
	export interface Report<S> {
		observer: Observer;
		snapshot: S;
	}
	export type Reset = "decrease" | "replaced";
	export type Role = "publisher" | "viewer";
	export interface Sample<S> {
		age: number;
		current: Report<S>;
		interval: number;
		previous?: Report<S>;
	}
	export interface Thresholds {
		publisher: {
			skipped: Limits;
		};
		stale: number;
		transport: {
			headroom: Limits;
			loss: Limits;
			rtt: Limits;
		};
		viewer: {
			errors: Limits;
			late: Limits;
			stalled: Limits;
		};
	}
	export type Verdict = "unknown" | "healthy" | "degraded" | "unhealthy";
	export function defaultThresholds(): Thresholds;
	export function publisher(sample: Sample<{
		[key: string]: unknown;
		renditions?: Record<string, {
			bitrate?: number & $brand<"u53">;
			bytes?: number & $brand<"u53">;
			frames?: number & $brand<"u53">;
			keyframes?: number & $brand<"u53">;
			skipped?: number & $brand<"u53">;
		}>;
		transport?: {
			age?: number & $brand<"u53">;
			lostBytes?: number & $brand<"u53">;
			lostPackets?: number & $brand<"u53">;
			rate?: number & $brand<"u53">;
			rtt?: number & $brand<"u53">;
		};
	}>, thresholds?: Thresholds): Health;
	export function viewer(sample: Sample<{
		[key: string]: unknown;
		renditions?: Record<string, {
			arrival?: {
				timestamp: number & $brand<"u53">;
				wall: number & $brand<"u53">;
			};
			bytes?: number & $brand<"u53">;
			decoded?: number & $brand<"u53">;
			errors?: number & $brand<"u53">;
			frames?: number & $brand<"u53">;
			late?: number & $brand<"u53">;
			latency?: number & $brand<"u53">;
			stalled?: number & $brand<"u53">;
			stalls?: number & $brand<"u53">;
			underruns?: number & $brand<"u53">;
		}>;
		transport?: {
			age?: number & $brand<"u53">;
			lostBytes?: number & $brand<"u53">;
			lostPackets?: number & $brand<"u53">;
			rate?: number & $brand<"u53">;
			rtt?: number & $brand<"u53">;
		};
	}>, thresholds?: Thresholds): Health;
}

export * as Net from "@moq/net";

export * as Signals from "@moq/signals";

export namespace Stats {
	export type Rendition = z.infer<typeof RenditionSchema>;
	export const RenditionSchema: z.ZodMiniObject<{
		bitrate: z.ZodMiniOptional<z.ZodMiniPipe<z.ZodMiniNullable<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>, z.ZodMiniTransform<number & z.$brand<"u53"> | undefined, number & z.$brand<"u53"> | null>>>;
		bytes: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>;
		frames: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>;
		keyframes: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>;
		skipped: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>;
	}, z.core.$strip>;
	export type Snapshot = z.infer<typeof SnapshotSchema>;
	export const SnapshotSchema: z.ZodMiniObject<{
		renditions: z.ZodMiniOptional<z.ZodMiniRecord<z.ZodMiniString<string>, z.ZodMiniObject<{
			bitrate: z.ZodMiniOptional<z.ZodMiniPipe<z.ZodMiniNullable<z.core.$ZodBranded<z.ZodMiniNumber<...>, "u53", "out">>, z.ZodMiniTransform<... | undefined, ... | null>>>;
			bytes: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>;
			frames: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>;
			keyframes: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>;
			skipped: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>;
		}, z.core.$strip>>>;
		transport: z.ZodMiniOptional<z.ZodMiniObject<{
			age: z.ZodMiniOptional<z.ZodMiniPipe<z.ZodMiniNullable<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>, z.ZodMiniTransform<... & ... | undefined, ... & ... | null>>>;
			lostBytes: z.ZodMiniOptional<z.ZodMiniPipe<z.ZodMiniNullable<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>, z.ZodMiniTransform<... & ... | undefined, ... & ... | null>>>;
			lostPackets: z.ZodMiniOptional<z.ZodMiniPipe<z.ZodMiniNullable<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>, z.ZodMiniTransform<... & ... | undefined, ... & ... | null>>>;
			rate: z.ZodMiniOptional<z.ZodMiniPipe<z.ZodMiniNullable<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>, z.ZodMiniTransform<... & ... | undefined, ... & ... | null>>>;
			rtt: z.ZodMiniOptional<z.ZodMiniPipe<z.ZodMiniNullable<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>, z.ZodMiniTransform<... & ... | undefined, ... & ... | null>>>;
		}, z.core.$strip>>;
	}, z.core.$loose>;
	export type Transport = z.infer<typeof TransportSchema>;
	export const TransportSchema: z.ZodMiniObject<{
		age: z.ZodMiniOptional<z.ZodMiniPipe<z.ZodMiniNullable<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>, z.ZodMiniTransform<number & z.$brand<"u53"> | undefined, number & z.$brand<"u53"> | null>>>;
		lostBytes: z.ZodMiniOptional<z.ZodMiniPipe<z.ZodMiniNullable<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>, z.ZodMiniTransform<number & z.$brand<"u53"> | undefined, number & z.$brand<"u53"> | null>>>;
		lostPackets: z.ZodMiniOptional<z.ZodMiniPipe<z.ZodMiniNullable<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>, z.ZodMiniTransform<number & z.$brand<"u53"> | undefined, number & z.$brand<"u53"> | null>>>;
		rate: z.ZodMiniOptional<z.ZodMiniPipe<z.ZodMiniNullable<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>, z.ZodMiniTransform<number & z.$brand<"u53"> | undefined, number & z.$brand<"u53"> | null>>>;
		rtt: z.ZodMiniOptional<z.ZodMiniPipe<z.ZodMiniNullable<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>, z.ZodMiniTransform<number & z.$brand<"u53"> | undefined, number & z.$brand<"u53"> | null>>>;
	}, z.core.$strip>;
}

export namespace Timeline {
	export class Consumer implements AsyncIterable<Event> {
		[asyncIterator](): AsyncIterator<Event>;
		close(): void;
		next(): Promise<Event | undefined>;
		static subscribe(broadcast: Consumer, archive: {
			replay?: RelativeBroadcast;
			replayEpoch?: ReferenceEpoch;
			store?: string;
			timelines: z.ZodMiniRecord<z.ZodMiniString<string>, z.ZodMiniObject<{
				duration: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>;
				track: z.ZodMiniString<string>;
			}, z.core.$strip>>;
			timescale: number & $brand<"u53">;
			version?: number & $brand<"u53">;
		}, track: string): Consumer;
	}
	export type Entry = Omit<Record, "pts" | "duration"> & {
		duration: Micro;
		pts: Micro;
	};
	export type Event = {
		push: {
			entry: Entry;
			index: number;
		};
	} | {
		pop: Json.Window.Span;
	} | {
		skip: Json.Window.Span;
	};
	export interface Position {
		frame?: number;
		group: number;
	}
	export interface Record {
		duration?: number;
		end: Position;
		keyframe?: boolean;
		pts?: number;
		sequence: number;
		start: Position;
	}
	export const SUFFIX: ".timeline.z";
}
```
