# `@moq/flate`

```ts
export type Compression = "none" | "deflate";

export const DEFAULT_LEVEL: 6;

export class Decoder {
	constructor();
	frame(slice: Uint8Array): Uint8Array;
}

export class Encoder {
	constructor(options?: EncoderOptions);
	static bound(len: number): number;
	frame(payload: Uint8Array): Uint8Array;
}

export interface EncoderOptions {
	level?: number;
}

export namespace Snapshot {
	export interface Config {
		compression?: Compression;
	}
	export namespace Consumer {
		export type Config = Config & {
			track: Moq.Track.Subscriber;
		};
	}
	export class Consumer {
		constructor(config: Config);
		[asyncIterator](): AsyncIterator<Timed<Uint8Array<ArrayBufferLike>>>;
		latest(): Promise<Timed<Uint8Array<ArrayBufferLike>> | undefined>;
		next(): Promise<Timed<Uint8Array<ArrayBufferLike>> | undefined>;
	}
	export namespace Producer {
		export type Config = Config & {
			track: Moq.Track.Producer;
		};
	}
	export class Producer {
		constructor(config: Config);
		finish(): void;
		update({ ... }: Timed<Uint8Array<ArrayBufferLike>>): void;
	}
}

export namespace Stream {
	export interface Config {
		compression?: Compression;
	}
	export namespace Consumer {
		export type Config = Config & {
			track: Moq.Track.Subscriber;
		};
	}
	export class Consumer {
		constructor(config: Config);
		[asyncIterator](): AsyncIterator<Timed<Uint8Array<ArrayBufferLike>>>;
		next(): Promise<Timed<Uint8Array<ArrayBufferLike>> | undefined>;
	}
	export namespace Producer {
		export type Config = Config & {
			track: Moq.Track.Producer;
		};
	}
	export class Producer {
		constructor(config: Config);
		append({ ... }: Timed<Uint8Array<ArrayBufferLike>>): void;
		finish(): void;
	}
	export class Rolled extends Error {
		constructor();
	}
}
```
