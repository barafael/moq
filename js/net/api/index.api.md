# `@moq/net`

```ts
export namespace Announce {
	export interface Announce {
		captures: Pattern[] | undefined;
		prefix: Valid;
		route: Route;
	}
	export class Consumer {
		[asyncIterator](): AsyncGenerator<Event, void, undefined>;
		close(abort?: Error): void;
		get closed(): GetPromise<Error | null>;
		next(): Promise<Event | undefined>;
		get synced(): Getter<boolean>;
	}
	export type Event = {
		kind: "start" | "update" | "restart" | "end";
	} & Announce;
	export interface Options {
		hidden?: boolean;
	}
}

export namespace Auth {
	export interface Auth {
		close(): void;
		readonly closed: PromiseLike<Error | null>;
		readonly grant: Getter<Grant | undefined>;
	}
	export class Duplicate extends Error {
		constructor(message?: string);
	}
	export interface Grant {
		expires?: number;
		publish: Patterns;
		subscribe: Patterns;
	}
	export class Handle {
		constructor();
		abort(err?: Error): void;
		add(token: string | Uint8Array<ArrayBufferLike>): Promise<Auth>;
		get grant(): Getter<Grant | undefined>;
	}
	export class Unsupported extends Error {
		constructor(message?: string);
		readonly code: SessionCode;
	}
}

export namespace Bandwidth {
	export class Allocator implements Handle {
		constructor(estimate?: Getter<number | undefined>);
		close(): void;
		reserve(track: Demand, max: number): Reservation;
		static unlimited(): Allocator;
	}
	export interface Handle {
		reserve(track: Demand, max: number): Reservation;
	}
	export class Reservation implements Disposable {
		[dispose](): void;
		close(): void;
		get grant(): Getter<number | undefined>;
		peek(): number | undefined;
		update(max: number): void;
	}
}

export namespace Broadcast {
	export class Consumer {
		protected constructor(shared?: undefined);
		clone(): Consumer;
		close(): void;
		get closed(): GetPromise<null>;
		get epoch(): Valid | undefined;
		get path(): Valid;
		protected shareState(): never;
		track(name: string): Consumer;
		tracks(): Tracks;
	}
	export class Producer {
		constructor();
		close(): void;
		get closed(): GetPromise<null>;
		consume(): Consumer;
		createTrack(name: string, info?: Partial<Info>): Producer;
		removeTrack(name: string): void;
		track(name: string): Consumer;
	}
	export class Tracks {
		[asyncIterator](): AsyncGenerator<TracksEvent, void, undefined>;
		close(): void;
		next(): Promise<TracksEvent | undefined>;
	}
	export type TracksEvent = {
		info: track.Info;
		kind: "added";
		name: string;
	} | {
		kind: "synced";
	} | {
		kind: "removed";
		name: string;
	} | {
		kind: "finished";
		name: string;
	};
}

export namespace Connection {
	export interface AcceptProps {
		consume?: Producer;
		implementation?: string;
		publish?: Consumer;
		transport: WebTransport;
		url: URL;
		version?: number;
	}
	export type Backoff = {
		healthy?: Milli;
		initial?: Milli;
		max?: Milli;
		multiplier?: number;
		timeout?: Milli;
	};
	export interface CertificateHash {
		algorithm?: "sha-256";
		value: string | BufferSource;
	}
	export interface ConnectProps {
		auth?: Handle;
		consume?: Producer;
		implementation?: string;
		publish?: Consumer;
		signal?: AbortSignal;
		timeout?: Milli;
		tokens?: readonly string[];
		transport?: WebTransport;
		url: URL;
		versions?: readonly string[];
		websocket?: WebSocketProps;
		webtransport?: WebTransportProps;
	}
	export interface Established {
		abort(): void;
		announced(scope?: Pattern, options?: Options): Consumer;
		readonly auth: Handle;
		close(): Promise<void>;
		closed: Promise<Error | null>;
		readonly peerImplementation: Promise<string | undefined>;
		readonly probe: Getter<Probe>;
		stats(): Promise<Stats>;
		readonly transport: Transport;
		readonly url: URL;
		readonly version: string;
	}
	export interface Extensions {
		auth?: boolean;
		epoch?: boolean;
		solicit?: boolean;
	}
	export interface Goaway {
		handover?: Milli;
		redirect?: Redirect;
	}
	export interface Probe {
		estimatedRecvRate?: number;
		rtt?: Milli;
	}
	export interface Props {
		consume?: Producer;
		delay?: Backoff;
		enabled?: boolean | Signal<boolean>;
		goaway?: Goaway;
		publish?: Consumer;
		share?: boolean;
		signal?: undefined;
		timeout?: Milli;
		tokens?: readonly string[] | Signal<readonly string[]>;
		transport?: undefined;
		url?: URL | Signal<URL | undefined>;
		versions?: readonly string[] | Signal<readonly string[] | undefined>;
		websocket?: WebSocketProps;
		webtransport?: WebTransportProps;
	}
	export type Redirect = "follow" | "same-host" | "ignore";
	export interface Stats {
		bytesLost?: number;
		bytesReceived?: number;
		bytesSent?: number;
		estimatedSendRate?: number;
		packetsLost?: number;
		packetsReceived?: number;
		packetsSent?: number;
		rtt?: Milli;
	}
	export type Status = "connecting" | "connected" | "disconnected";
	export type Transport = "webtransport" | "websocket" | "webrtc";
	export interface WebSocketProps {
		delay?: Milli;
		enabled?: boolean;
		url?: URL;
	}
	export interface WebTransportProps extends Omit<WebTransportOptions, "serverCertificateHashes" | "protocols"> {
		serverCertificate?: string | BufferSource;
		serverCertificateHashes?: CertificateHash[];
	}
}

export class Connection {
	constructor(props?: Props);
	static readonly accept: ({ ... }: AcceptProps) => Promise<Established>;
	announced(scope?: Pattern, options?: Options): Consumer;
	readonly auth: Getter<Handle | undefined>;
	readonly bandwidth: Getter<Handle | undefined>;
	static readonly certificateHash: (cert: string | BufferSource) => Promise<Uint8Array<ArrayBuffer>>;
	close(abort?: Error): void;
	get closed(): GetPromise<Error | null>;
	static readonly connect: ({ ... }: ConnectProps) => Promise<Established>;
	enabled: Signal<boolean>;
	readonly error: Getter<Error | undefined>;
	static readonly isWebTransportSupported: () => boolean;
	readonly origin: Getter<Table | undefined>;
	readonly probe: Getter<Probe | undefined>;
	stats(): Promise<Stats | undefined>;
	readonly status: Getter<Status>;
	readonly transport: Getter<Transport | undefined>;
	url: Signal<URL | undefined>;
	versions: Signal<readonly string[] | undefined>;
}

export namespace Epoch {
	export type Valid = string & {
		readonly _brand: "epoch";
	};
	export function mint(): Valid;
	export function parse(text: string): Valid;
}

export namespace Error {
	export class ConnectTimeout extends Error {
		constructor(timeout: Milli);
		readonly timeout: Milli;
	}
	export class FrameTooLarge extends Stream {
		constructor(options?: {
			cause?: unknown;
		});
	}
	export class GroupTooLarge extends Stream {
		constructor(options?: {
			cause?: unknown;
		});
	}
	export class NoStreamCredit extends Error {
		constructor(options?: ErrorOptions);
	}
	export class NotFound extends Stream {
		constructor(what: string, options?: {
			cause?: unknown;
		});
	}
	export class ProtocolViolation extends Error {
		constructor(message: string, options?: {
			cause?: unknown;
			code?: SessionCode;
		});
		readonly code: SessionCode;
	}
	export class RefusedRedirect extends Error {
		constructor(reason: string);
	}
	export class Session extends Error {
		constructor(code: SessionCode, options?: {
			cause?: unknown;
			reason?: string;
		});
		readonly code: SessionCode;
	}
	export class Stream extends Error {
		constructor(code: StreamCode, options?: StreamOptions);
		readonly code: StreamCode;
	}
	export interface StreamOptions {
		cause?: unknown;
		message?: string;
		reason?: string;
	}
	export class TimestampMismatch extends Stream {
		constructor(options?: {
			cause?: unknown;
		});
	}
	export class TooFarBehind extends Stream {
		constructor(options?: {
			cause?: unknown;
		});
	}
}

export namespace Group {
	export class Consumer {
		close(abort?: Error): void;
		get closed(): GetPromise<Error | null>;
		get done(): boolean;
		get frameCount(): number;
		get isClosed(): boolean;
		readBool(): Promise<boolean | undefined>;
		readFrame(): Promise<Frame | undefined>;
		readFrameSequence(options?: ReadOptions): Promise<{
			sequence: number;
		} & Frame | undefined>;
		readJson(): Promise<unknown>;
		readString(): Promise<string | undefined>;
		readable(): Promise<void>;
		readonly sequence: number;
		tryReadFrame(): Frame | undefined;
		tryReadFrameSequence(): {
			sequence: number;
		} & Frame | undefined;
	}
	export interface Frame {
		payload: Uint8Array;
		timestamp?: Timestamp;
	}
	export interface Info {
		sequence: number;
	}
	export const MAX_GROUP_CACHE_BYTES: number;
	export const MAX_GROUP_FRAMES: 8192;
	export class Producer {
		constructor(sequence: number);
		close(abort?: Error): void;
		get closed(): GetPromise<Error | null>;
		consume(): Consumer;
		get isClosed(): boolean;
		readonly sequence: number;
		writeFrame(frame: Frame): void;
	}
	export interface ReadOptions {
		from?: number;
	}
}

export const IMPLEMENTATION: string;

export namespace Origin {
	export class Consumer {
		announced(scope?: Pattern, options?: Options): Consumer;
		get closed(): GetPromise<Error | null>;
		get discovery(): Getter<boolean | undefined>;
		request(path: Valid, options?: RequestOptions): Requesting;
	}
	export type Cost = {
		zero: bigint;
	};
	export type Cost = bigint;
	export class Dynamic {
		close(): void;
		readonly prefix: Valid;
		requested(): AsyncIterableIterator<Request>;
		get route(): Route;
	}
	export type Hop = z.infer<typeof HopSchema>;
	export const HopSchema: z.core.$ZodBranded<z.ZodMiniBigInt<bigint>, "Hop", "out">;
	export class Prepared {
		[dispose](): void;
		readonly broadcast: Producer;
		close(): void;
		publish(): Publication;
	}
	export class Producer implements Table {
		constructor();
		announced(scope?: Pattern, options?: Options): Consumer;
		close(abort?: Error): void;
		get closed(): GetPromise<Error | null>;
		consume(): Consumer;
		createBroadcast(path: Valid): Publication;
		get discovery(): Getter<boolean | undefined>;
		dynamic(prefix: Valid): Dynamic;
		request(path: Valid, options?: RequestOptions): Requesting;
		scope(root: Valid, patterns: Patterns): Producer;
	}
	export class Publication {
		announce(route?: Route | {
			auth?: Auth;
			cost?: bigint;
			epoch?: Valid;
			hops?: (bigint & $brand<"Hop">)[];
		}): void;
		readonly broadcast: Producer;
		get route(): Route | undefined;
		unannounce(): void;
	}
	export class Request {
		readonly path: Valid;
		prepare(epoch: Valid): Prepared;
		reject(err: Error): void;
	}
	export interface RequestOptions {
		announced?: boolean;
		epoch?: Valid;
	}
	export class Requesting {
		readonly active: Getter<Consumer | undefined>;
		close(): void;
		readonly closed: GetPromise<Error | null>;
		readonly path: Valid;
	}
	export interface Route {
		auth?: Auth;
		cost: bigint;
		epoch?: Valid;
		hops: (bigint & $brand<"Hop">)[];
	}
	export type Route = {
		default: Route;
		normalize(route?: Route | {
			auth?: Auth;
			cost?: bigint;
			epoch?: Valid;
			hops?: readonly (bigint & $brand<"Hop">)[];
		}): Route;
	};
	export interface Table {
		announced(scope?: Pattern, options?: Options): Consumer;
		readonly closed: GetPromise<Error | null>;
		createBroadcast(path: Valid): Publication;
		readonly discovery: Getter<boolean | undefined>;
		dynamic(prefix: Valid): Dynamic;
		request(path: Valid, options?: RequestOptions): Requesting;
	}
	export function isAnonymous(route: Route): boolean;
}

export namespace Path {
	export { InvalidPattern } from "@moq/pattern";
	export const MAX_PARTS: 32;
	export { Pattern } from "@moq/pattern";
	export { Patterns } from "@moq/pattern";
	export type Relative = string & {
		__brand: "Relative";
	};
	export { Segment } from "@moq/pattern";
	export { Specificity } from "@moq/pattern";
	export type Valid = string & {
		__brand: "Name";
	};
	export { compareSpecificity } from "@moq/pattern";
	export function decode(raw: string): Valid;
	export function empty(): Valid;
	export function encode(path: Valid): Valid;
	export function from(...paths: string[]): Valid;
	export function hasPrefix(prefix: Valid, path: Valid): boolean;
	export function join(path: Valid, other: Valid): Valid;
	export function normalizeRelative(rel: string): Relative;
	export function parts(path: Valid): string[];
	export function relative(target: Valid, base: Valid): Relative | undefined;
	export function resolve(base: Valid, rel: Relative): Valid;
	export function stripPrefix(prefix: Valid, path: Valid): Valid | null;
	export function tryResolve(base: Valid, rel: Relative): Valid | undefined;
}

export const SessionCode: ((code: number) => SessionCode) & {
	readonly AuthTokenCacheOverflow: SessionCode;
	readonly Cancel: SessionCode;
	readonly Expired: SessionCode;
	readonly GoawayTimeout: SessionCode;
	readonly Internal: SessionCode;
	readonly InvalidAuthority: SessionCode;
	readonly InvalidPath: SessionCode;
	readonly InvalidRequestId: SessionCode;
	readonly KeyValueFormatting: SessionCode;
	readonly MalformedAuthToken: SessionCode;
	readonly ProtocolViolation: SessionCode;
	readonly Timeout: SessionCode;
	readonly TooManyRequestUpdates: SessionCode;
	readonly TooManyRequests: SessionCode;
	readonly Unauthorized: SessionCode;
	readonly UnknownAuthTokenAlias: SessionCode;
	readonly Unsupported: SessionCode;
	readonly Version: SessionCode;
};

export type SessionCode = number & {
	readonly [SESSION_CODE]: true;
};

export * as Signals from "@moq/signals";

export const StreamCode: ((code: number) => StreamCode) & {
	readonly Cancel: StreamCode;
	readonly ControlTimeout: StreamCode;
	readonly DeliveryTimeout: StreamCode;
	readonly Evicted: StreamCode;
	readonly FrameTooLarge: StreamCode;
	readonly GoingAway: StreamCode;
	readonly GroupTooLarge: StreamCode;
	readonly Internal: StreamCode;
	readonly MalformedTrack: StreamCode;
	readonly NotFound: StreamCode;
	readonly Old: StreamCode;
	readonly SessionClosed: StreamCode;
	readonly TimestampMismatch: StreamCode;
	readonly TooFarBehind: StreamCode;
	readonly Unauthorized: StreamCode;
	readonly Unroutable: StreamCode;
};

export type StreamCode = number & {
	readonly [STREAM_CODE]: true;
};

export namespace Time {
	export const Timescale: ((unitsPerSecond: number) => Timescale) & {
		MICRO: Timescale;
		MILLI: Timescale;
		NANO: Timescale;
		SECOND: Timescale;
	};
	export type Timescale = number & {
		readonly _brand: "timescale";
	};
	export class Timestamp {
		constructor(value: number, scale: Timescale);
		as(scale: Timescale): number;
		asMicros(): number;
		asMillis(): number;
		static fromMicros(us: number): Timestamp;
		static fromMillis(ms: number): Timestamp;
		isBefore(other: Timestamp): boolean;
		static now(): Timestamp;
		readonly scale: Timescale;
		readonly value: number;
	}
}

export interface Timed<T> {
	at?: Timestamp;
	value: T;
}

export namespace Track {
	export type Bound = {
		excluded?: never;
		included: number;
	} | {
		excluded: number;
		included?: never;
	};
	export class Consumer {
		fetchGroup(sequence: number, options?: FetchGroupOptions): Promise<Consumer>;
		info(): Promise<Info>;
		readonly name: string;
		subscribe(options?: Subscription): Subscriber;
	}
	export interface Datagram {
		payload: Uint8Array;
		sequence: number;
		timestamp?: Timestamp;
	}
	export class Demand {
		get closed(): GetPromise<Error | null>;
		readonly name: string;
		get wanted(): Getter<number | undefined>;
	}
	export interface FetchGroupOptions {
		maxDelay?: Milli;
		priority?: number;
		signal?: AbortSignal;
	}
	export interface Groups {
		end?: Bound;
		start?: Bound;
	}
	export interface Info {
		compress?: boolean;
		maxAge?: Milli;
		priority: number;
		refresh?: number;
		timescale?: Timescale | null;
	}
	export type Order = "ascending" | "descending";
	export class Ordered {
		close(abort?: Error): void;
		get closed(): GetPromise<Error | null>;
		end(): Position | undefined;
		finished(): Promise<Position>;
		info(): Promise<Info>;
		largest(): Promise<Position | undefined>;
		readonly name: string;
		nextGroup(): Promise<Consumer | undefined>;
		readBool(): Promise<boolean | undefined>;
		readFrame(): Promise<{
			frame: number;
			group: number;
		} & Frame | undefined>;
		readJson(): Promise<unknown>;
		readString(): Promise<string | undefined>;
		recvDatagram(): Promise<Datagram | undefined>;
		get subscription(): Getter<Subscription | undefined>;
		tryNextGroup(): Consumer | undefined;
		update(options: Subscription): void;
		withRange(groups: Groups): this;
	}
	export interface Position {
		frame: number;
		group: number;
	}
	export class Producer {
		constructor(name: string);
		accept(info?: Partial<Info>): this;
		appendDatagram(timestamp: Timestamp | undefined, payload: Uint8Array): number;
		appendGroup(): Producer;
		close(abort?: Error): void;
		get closed(): GetPromise<Error | null>;
		demand(): Demand;
		finishAt(end: Position): void;
		info(): Promise<Info>;
		insertDatagram(sequence: number, timestamp: Timestamp | undefined, payload: Uint8Array): void;
		largest(): Position | undefined;
		readonly name: string;
		get priority(): number;
		subscribe(options?: Subscription): Subscriber;
		get subscription(): Getter<Subscription | undefined>;
		writeFrame(frame: Frame): void;
		writeGroup(group: Producer): void;
	}
	export class Request {
		accept(info?: Partial<Info>): Producer;
		demand(): Demand;
		readonly name: string;
		get priority(): number;
		reject(err?: Error): void;
		get subscription(): Readonly<Subscription>;
	}
	export class Subscriber {
		close(abort?: Error): void;
		get closed(): GetPromise<Error | null>;
		end(): Position | undefined;
		finished(): Promise<Position>;
		info(): Promise<Info>;
		largest(): Promise<Position | undefined>;
		readonly name: string;
		ordered(): Ordered;
		recvDatagram(): Promise<Datagram | undefined>;
		recvGroup(): Promise<Consumer | undefined>;
		get subscription(): Getter<Subscription | undefined>;
		tryRecvGroup(): Consumer | undefined;
		update(options: Subscription): void;
		withRange(groups: Groups): this;
	}
	export interface Subscription {
		groups?: Groups;
		maxDelay?: Milli;
		order?: Order;
		priority?: number;
	}
	export function infoDefaults(info?: Partial<Info>): Info;
}

export namespace Varint {
	export const MAX_U14: number;
	export const MAX_U30: number;
	export const MAX_U53: number;
	export const MAX_U6: number;
	export function decode(buf: Uint8Array): [number, Uint8Array<ArrayBufferLike>];
	export function decodeBigInt(buf: Uint8Array): [bigint, Uint8Array<ArrayBufferLike>];
	export function decodeLeadingOnes(buf: Uint8Array): [bigint, Uint8Array<ArrayBufferLike>];
	export function encode(v: number | bigint): Uint8Array;
	export function encodeLeadingOnes(v: number | bigint): Uint8Array;
	export function encodeLeadingOnesTo(dst: ArrayBuffer, v: number | bigint): Uint8Array;
	export function encodeTo(dst: ArrayBuffer, v: number | bigint): Uint8Array;
	export function size(v: number): number;
}
```
