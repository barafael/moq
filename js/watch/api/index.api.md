# `@moq/watch`

```ts
export type Arrival = {
	arrival: Milli;
	kind: "frame";
	late: boolean;
	timestamp: Milli;
} | {
	duration: Milli;
	kind: "gap";
	timestamp: Milli;
};

export interface Arrivals {
	[asyncIterator](): AsyncIterator<Arrival>;
	close(): void;
	next(): Promise<Arrival | undefined>;
}

export namespace Audio {
	export class Decoder {
		constructor(props: DecoderProps);
		arrivals(): Arrivals;
		close(): void;
		readonly in: Readonlys<DecoderInput>;
		readonly out: Readonlys<DecoderOutput>;
		reset(): void;
		readonly source: Source;
		static supported: (config: {
			bitrate?: number & $brand<"u53">;
			broadcast?: RelativeBroadcast;
			codec: string;
			container: {
				kind: "unknown";
				raw: {
					[key: string]: unknown;
					kind: string;
				};
			} | {
				kind: "legacy";
			} | {
				init: string;
				kind: "cmaf";
			} | {
				kind: "loc";
			};
			delay?: number & $brand<"u53">;
			description?: string;
			enabled?: boolean;
			epoch?: ReferenceEpoch;
			jitter?: number & $brand<"u53">;
			label?: string;
			layout?: string;
			numberOfChannels: number & $brand<"u53">;
			sampleRate: number & $brand<"u53">;
			track?: string;
			warmup?: number & $brand<"u53">;
		}) => Promise<boolean>;
		readonly sync: Sync;
	}
	export type DecoderInput = {
		enabled: Getter<boolean>;
	};
	export type DecoderOutput = {
		buffered: Signal<Container.BufferedRanges>;
		context: Signal<AudioContext | undefined>;
		error: Signal<Error | undefined>;
		root: Signal<AudioNode | undefined>;
		sampleRate: Signal<number | undefined>;
		stalled: Signal<boolean>;
		stats: Signal<Stats | undefined>;
		timestamp: Signal<Milli | undefined>;
	};
	export type DecoderProps = Inputs<DecoderInput> & {
		source: Source;
		sync: Sync;
	};
	export class Emitter {
		constructor(props: EmitterProps);
		close(): void;
		readonly in: Readonlys<EmitterInput>;
		readonly out: Readonlys<EmitterOutput>;
		readonly source: Decoder;
	}
	export type EmitterInput = {
		muted: Getter<boolean>;
		paused: Getter<boolean>;
		volume: Getter<number>;
	};
	export type EmitterOutput = {
		enabled: Signal<boolean>;
	};
	export type EmitterProps = Inputs<EmitterInput> & {
		source: Decoder;
	};
	export class Source {
		constructor(props?: Inputs<SourceInput>);
		close(): void;
		readonly in: Readonlys<SourceInput>;
		readonly out: Readonlys<SourceOutput>;
	}
	export type SourceInput = {
		broadcast: Getter<Broadcast | undefined>;
		supported: Getter<Supported | undefined>;
		target: Getter<Target | undefined>;
	};
	export type SourceOutput = {
		available: Signal<Record<string, Catalog.AudioConfig>>;
		catalog: Signal<Catalog.Audio | undefined>;
		config: Signal<Catalog.AudioConfig | undefined>;
		jitter: Signal<Milli | undefined>;
		track: Signal<string | undefined>;
	};
	export interface Stats {
		bytesReceived: number;
		underruns: number;
	}
	export type Supported = (config: Catalog.AudioConfig) => Promise<boolean>;
	export type Target = {
		name?: string;
	};
}

export class Broadcast {
	constructor(props?: Inputs<BroadcastInput>);
	close(): void;
	readonly in: Readonlys<BroadcastInput>;
	readonly out: Readonlys<BroadcastOutput>;
}

export type BroadcastInput = {
	announced: Getter<boolean>;
	catalog: Getter<Catalog.Root | undefined>;
	catalogFormat: Getter<CatalogFormat | undefined>;
	enabled: Getter<boolean>;
	name: Getter<Moq.Path.Valid>;
	origin: Getter<Moq.Origin.Table | undefined>;
};

export type BroadcastOutput = {
	active: Signal<Moq.Broadcast.Consumer | undefined>;
	catalog: Signal<Catalog.Root | undefined>;
	error: Signal<Error | undefined>;
	status: Signal<"offline" | "loading" | "live" | "error" | "unsupported">;
};

export { BufferedRange } from "@moq/hang";

export { BufferedRanges } from "@moq/hang";

export const CATALOG_FORMATS: readonly ["hang", "msf", "hangz", "manual"];

export type CatalogFormat = typeof CATALOG_FORMATS[number];

export * as Clock from "@moq/clock";

export type Delay = "instant" | "auto" | Milli;

export * as Hang from "@moq/hang";

export * as Net from "@moq/net";

export class Player {
	constructor(props?: PlayerProps);
	readonly audio: Decoder;
	readonly broadcast: Broadcast;
	close(): void;
	readonly emitter: Emitter;
	readonly in: Readonlys<Omit<PlayerInput, "video" | "audio" | "text">> & {
		audio: Readonlys<{
			muted: Getter<boolean>;
			target: Getter<Target | undefined>;
			volume: Getter<number>;
		}>;
		text: Readonlys<{
			container: Getter<HTMLElement | undefined>;
			target: Getter<string | undefined>;
		}>;
		video: Readonlys<{
			backend: Getter<Backend>;
			canvas: Getter<HTMLCanvasElement | undefined>;
			target: Getter<Target | undefined>;
			visible: Getter<Visible>;
		}>;
	};
	readonly renderer: Renderer;
	reset(): void;
	readonly sync: Sync;
	readonly text: Source;
	readonly textRenderer: Renderer;
	readonly video: Decoder;
}

export type PlayerInput = {
	announced: Getter<boolean>;
	audio: {
		muted: Getter<boolean>;
		target: Getter<Audio.Target | undefined>;
		volume: Getter<number>;
	};
	buffer: Getter<Milli>;
	catalog: Getter<Catalog.Root | undefined>;
	catalogFormat: Getter<CatalogFormat | undefined>;
	delay: Getter<Delay>;
	enabled: Getter<boolean>;
	name: Getter<Moq.Path.Valid>;
	origin: Getter<Moq.Origin.Table | undefined>;
	paused: Getter<boolean>;
	probe: Getter<Moq.Connection.Probe | undefined>;
	text: {
		container: Getter<HTMLElement | undefined>;
		target: Getter<string | undefined>;
	};
	video: {
		backend: Getter<Video.Backend>;
		canvas: Getter<HTMLCanvasElement | undefined>;
		target: Getter<Video.Target | undefined>;
		visible: Getter<Video.Visible>;
	};
};

export type PlayerProps = Inputs<Omit<PlayerInput, "video" | "audio" | "text">> & { [K in "video" | "audio" | "text"]?: Inputs<PlayerInput[K]> };

export * as Signals from "@moq/signals";

export class Sync {
	constructor(props?: Inputs<SyncInput>);
	close(): void;
	readonly in: Readonlys<SyncInput>;
	now(): Milli | undefined;
	readonly out: Readonlys<SyncOutput>;
	reset(): void;
}

export type SyncInput = {
	buffer: Getter<Milli>;
	delay: Getter<Delay>;
};

export type SyncOutput = {
	buffered: Signal<boolean>;
	delay: Signal<Milli>;
	instant: Signal<boolean>;
	maxDelay: Signal<Milli>;
	reference: Signal<number | undefined>;
	timestamp: Signal<Milli | undefined>;
};

export namespace Text {
	export class Renderer {
		constructor(props: RendererProps);
		close(): void;
		readonly in: Readonlys<RendererInput>;
		readonly source: Source;
		readonly sync: Sync;
	}
	export type RendererInput = {
		container: Getter<HTMLElement | undefined>;
		enabled: Getter<boolean>;
	};
	export type RendererProps = Inputs<RendererInput> & {
		source: Source;
		sync: Sync;
	};
	export class Source {
		constructor(props?: Inputs<SourceInput>);
		close(): void;
		readonly in: Readonlys<SourceInput>;
		readonly out: Readonlys<SourceOutput>;
	}
	export type SourceInput = {
		broadcast: Getter<Broadcast | undefined>;
		target: Getter<string | undefined>;
	};
	export type SourceOutput = {
		available: Signal<Record<string, Catalog.TextConfig>>;
		catalog: Signal<Catalog.Text | undefined>;
		config: Signal<Catalog.TextConfig | undefined>;
		track: Signal<string | undefined>;
	};
	export function renderable(config: {
		broadcast?: RelativeBroadcast;
		container: {
			kind: "unknown";
			raw: {
				[key: string]: unknown;
				kind: string;
			};
		} | {
			kind: "legacy";
		} | {
			init: string;
			kind: "cmaf";
		} | {
			kind: "loc";
		};
		delay?: number & $brand<"u53">;
		epoch?: ReferenceEpoch;
		format: string;
		jitter?: number & $brand<"u53">;
		label?: string;
		lang?: string;
		role: string;
		track?: string;
	}): boolean;
}

export namespace Video {
	export { Backend } from "@moq/video";
	export class Decoder {
		constructor(props: DecoderProps);
		close(): void;
		readonly in: Readonlys<DecoderInput>;
		readonly out: Readonlys<DecoderOutput>;
		readonly source: Source;
		static supported: (config: {
			bitrate?: number & $brand<"u53">;
			broadcast?: RelativeBroadcast;
			codec: string;
			codedHeight?: number & $brand<"u53">;
			codedWidth?: number & $brand<"u53">;
			color?: {
				contentLight?: {
					maxCll: number;
					maxFall: number;
				};
				mastering?: {
					blue: {
						x: number;
						y: number;
					};
					green: {
						x: number;
						y: number;
					};
					maxLuminance: number;
					minLuminance: number;
					red: {
						x: number;
						y: number;
					};
					white: {
						x: number;
						y: number;
					};
				};
				matrix?: number & $brand<"u8">;
				primaries?: number & $brand<"u8">;
				range?: "limited" | "full";
				transfer?: number & $brand<"u8">;
			};
			container: {
				kind: "unknown";
				raw: {
					[key: string]: unknown;
					kind: string;
				};
			} | {
				kind: "legacy";
			} | {
				init: string;
				kind: "cmaf";
			} | {
				kind: "loc";
			};
			delay?: number & $brand<"u53">;
			description?: string;
			displayAspectHeight?: number & $brand<"u53">;
			displayAspectWidth?: number & $brand<"u53">;
			enabled?: boolean;
			epoch?: ReferenceEpoch;
			framerate?: number;
			interlaced?: boolean;
			jitter?: number & $brand<"u53">;
			label?: string;
			optimizeForLatency?: boolean;
			preference?: number;
			projection?: {
				kind: "unknown";
				raw: {
					[key: string]: unknown;
					kind: string;
				};
			} | {
				kind: "rectilinear";
			} | {
				fov?: number;
				kind: "equirectangular";
			};
			reorder?: number & $brand<"u8">;
			stereo?: string;
			track?: string;
			warmup?: number & $brand<"u53">;
		}) => Promise<boolean>;
		readonly sync: Sync;
	}
	export type DecoderInput = {
		enabled: Getter<boolean>;
	};
	export type DecoderOutput = {
		buffered: Signal<Container.BufferedRanges>;
		display: Signal<{
			height: number;
			width: number;
		} | undefined>;
		error: Signal<Error | undefined>;
		frame: Signal<VideoFrame | undefined>;
		interlaced: Signal<boolean>;
		jitter: Signal<Milli | undefined>;
		stalled: Signal<boolean>;
		stats: Signal<Stats | undefined>;
		timestamp: Signal<Milli | undefined>;
	};
	export type DecoderProps = Inputs<DecoderInput> & {
		source: Source;
		sync: Sync;
	};
	export class Renderer {
		constructor(props: RendererProps);
		close(): void;
		readonly decoder: Decoder;
		readonly in: Readonlys<RendererInput>;
		readonly out: Readonlys<{
			error: Getter<Error | undefined>;
			frame: Getter<VideoFrame | undefined>;
			timestamp: Getter<Milli | undefined>;
			visible: Getter<boolean>;
		}>;
	}
	export type RendererInput = {
		backend: Getter<Video.Backend>;
		canvas: Getter<HTMLCanvasElement | undefined>;
		visible: Getter<Visible>;
	};
	export type RendererProps = Inputs<RendererInput> & {
		decoder: Decoder;
	};
	export class Source {
		constructor(props?: Inputs<SourceInput>);
		close(): void;
		readonly in: Readonlys<SourceInput>;
		readonly out: Readonlys<SourceOutput>;
	}
	export type SourceInput = {
		broadcast: Getter<Broadcast | undefined>;
		probe: Getter<Moq.Connection.Probe | undefined>;
		supported: Getter<Supported | undefined>;
		target: Getter<Target | undefined>;
	};
	export type SourceOutput = {
		available: Signal<Record<string, Catalog.VideoConfig>>;
		catalog: Signal<Catalog.Video | undefined>;
		config: Signal<Catalog.VideoConfig | undefined>;
		error: Signal<Error | undefined>;
		track: Signal<string | undefined>;
	};
	export interface Stats {
		bytesReceived: number;
		frameCount: number;
	}
	export type Supported = (config: Catalog.VideoConfig) => Promise<boolean>;
	export type Target = {
		bitrate?: number;
		height?: number;
		name?: string;
		pixels?: number;
		width?: number;
	};
	export type Visible = "never" | "always" | string & {};
}

export function assets(url: string | URL): void;
```
