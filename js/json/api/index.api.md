# `@moq/json`

```ts
export type Compression = "none" | "deflate";

export class Desync extends Error {
	constructor();
}

export interface Diff {
	forcedSnapshot: boolean;
	patch: unknown;
}

export class DuplicateKey extends Error {
	constructor();
}

export class MissingSnapshot extends Error {
	constructor();
}

export namespace Snapshot {
	export interface Config<T> {
		compression?: Compression;
		deltas?: boolean;
		initial?: T;
		schema?: ZodMiniType<T, unknown, $ZodTypeInternals<T, unknown>>;
	}
	export namespace Consumer {
		export type Config<T> = Pick<Config<T>, "schema" | "compression"> & {
			track: Moq.Track.Subscriber;
		};
	}
	export class Consumer<T> {
		constructor(config: Config<T>);
		[asyncIterator](): AsyncIterator<Timed<T>>;
		latest(): Promise<Timed<T> | undefined>;
		next(): Promise<Timed<T> | undefined>;
	}
	export class Decoder<T> {
		constructor(config?: Pick<Config<T>, "schema" | "compression">);
		decode(): T | undefined;
		delta(payload: Uint8Array): void;
		snapshot(payload: Uint8Array): void;
		get value(): unknown;
	}
	export interface Encoded {
		keyframe: boolean;
		payload: Uint8Array;
	}
	export class Encoder<T> {
		constructor(config?: Config<T>);
		reset(): void;
		snapshot(): Pending | undefined;
		update(value: T): Pending | undefined;
		get value(): T | undefined;
	}
	export { MissingSnapshot };
	export interface Pending extends Encoded {
		commit(): void;
	}
	export namespace Producer {
		export type Config<T> = Config<T> & {
			track: Moq.Track.Producer;
		};
	}
	export class Producer<T> {
		constructor(config: Config<T>);
		cut(): void;
		finish(): void;
		mutate(fn: (value: T) => void, at?: Timestamp): void;
		update({ ... }: Timed<T>): void;
	}
}

export namespace Stream {
	export interface Config<T = unknown> {
		compression?: Compression;
		schema?: ZodMiniType<T, unknown, $ZodTypeInternals<T, unknown>>;
	}
	export namespace Consumer {
		export type Config<T> = Config<T> & {
			track: Moq.Track.Subscriber;
		};
	}
	export class Consumer<T> {
		constructor(config: Config<T>);
		[asyncIterator](): AsyncIterator<Timed<T>>;
		next(): Promise<Timed<T> | undefined>;
	}
	export class Decoder<T> {
		constructor(config?: Config<T>);
		decode(payload: Uint8Array): T;
		reset(): void;
	}
	export { Desync };
	export class Encoder<T> {
		constructor(config?: Config<T>);
		encode(value: T): Pending;
		reset(): void;
	}
	export interface Pending {
		commit(): void;
		payload: Uint8Array;
	}
	export namespace Producer {
		export type Config<T> = Config<T> & {
			track: Moq.Track.Producer;
		};
	}
	export class Producer<T> {
		constructor(config: Config<T>);
		append({ ... }: Timed<T>): void;
		finish(): void;
	}
	export class Rolled extends Error {
		constructor();
	}
}

export namespace Window {
	export interface Config {
		checkpointRecords?: number;
		compression?: boolean;
		ops?: boolean;
	}
	export class Consumer<T> {
		constructor(config: ConsumerConfig);
		[asyncIterator](): AsyncIterator<Timed<Event<T>>>;
		next(): Promise<Timed<Event<T>> | undefined>;
		get offset(): number;
	}
	export type ConsumerConfig = DecoderConfig & {
		track: Moq.Track.Subscriber;
	};
	export class Decoder<T> {
		constructor(config?: DecoderConfig);
		group(): Group;
		next(): Event<T> | undefined;
		get offset(): number;
	}
	export interface DecoderConfig {
		compression?: boolean;
	}
	export interface Encoded {
		keyframe: boolean;
		payload: Uint8Array;
	}
	export class Encoder<T> {
		constructor(config?: Config);
		get end(): number;
		get offset(): number;
		pop(count: number): Pending | undefined;
		push(value: T): Pending;
		reset(): void;
		snapshot(): Pending;
		get window(): T[];
	}
	export type Event<T> = {
		push: {
			index: number;
			value: T;
		};
	} | {
		pop: Span;
	} | {
		skip: Span;
	};
	export interface Group {
		decode(payload: Uint8Array): void;
	}
	export interface Pending extends Encoded {
		commit(): void;
	}
	export class Producer<T> {
		constructor(config: ProducerConfig);
		cut(): void;
		finish(): void;
		get offset(): number;
		pop(count: number, at?: Timestamp): void;
		push({ ... }: Timed<T>): void;
		get window(): T[];
	}
	export type ProducerConfig = Config & {
		track: Moq.Track.Producer;
	};
	export interface Span {
		end: number;
		start: number;
	}
}

export function deepEqual(a: unknown, b: unknown): boolean;

export function diff(oldVal: unknown, newVal: unknown): Diff;

export function merge(target: unknown, patch: unknown): unknown;
```
