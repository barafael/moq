# `@moq/hang/catalog`

```ts
export const ARCHIVE_VERSION: 3;

export type Archive = z.infer<typeof ArchiveSchema>;

export const ArchiveSchema: z.ZodMiniObject<{
	replay: z.ZodMiniOptional<z.ZodMiniType<RelativeBroadcast, string, z.core.$ZodTypeInternals<RelativeBroadcast, string>>>;
	replayEpoch: z.ZodMiniOptional<z.ZodMiniType<ReferenceEpoch, string, z.core.$ZodTypeInternals<ReferenceEpoch, string>>>;
	store: z.ZodMiniOptional<z.ZodMiniURL>;
	timelines: z.ZodMiniRecord<z.ZodMiniString<string>, z.ZodMiniObject<{
		duration: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>;
		track: z.ZodMiniString<string>;
	}, z.core.$strip>>;
	timescale: z.ZodMiniDefault<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>;
	version: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>;
}, z.core.$strip>;

export namespace Audio {
	export type Audio = z.infer<typeof AudioSchema>;
	export type AudioConfig = z.infer<typeof AudioConfigSchema>;
	export const AudioConfigSchema: z.ZodMiniObject<{
		bitrate: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>;
		broadcast: z.ZodMiniOptional<z.ZodMiniType<RelativeBroadcast, string, z.core.$ZodTypeInternals<RelativeBroadcast, string>>>;
		codec: z.ZodMiniString<string>;
		container: z.ZodMiniDefault<z.ZodMiniUnion<readonly [z.ZodMiniDiscriminatedUnion<[z.ZodMiniObject<{
			kind: z.ZodMiniLiteral<...>;
		}, z.core.$strip>, z.ZodMiniObject<{
			init: z.ZodMiniBase64;
			kind: z.ZodMiniLiteral<...>;
		}, z.core.$strip>, z.ZodMiniObject<{
			kind: z.ZodMiniLiteral<...>;
		}, z.core.$strip>], "kind">, z.ZodMiniPipe<z.ZodMiniObject<{
			kind: z.ZodMiniString<string>;
		}, z.core.$loose>, z.ZodMiniTransform<{
			kind: "unknown";
			raw: {
				[x: string]: unknown;
				kind: string;
			};
		}, {
			[x: string]: unknown;
			kind: string;
		}>>]>>;
		delay: z.ZodMiniOptional<z.ZodMiniPipe<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">, z.ZodMiniTransform<number & z.$brand<"u53"> | undefined, number & z.$brand<"u53">>>>;
		description: z.ZodMiniOptional<z.ZodMiniString<string>>;
		enabled: z.ZodMiniOptional<z.ZodMiniBoolean<boolean>>;
		epoch: z.ZodMiniOptional<z.ZodMiniType<ReferenceEpoch, string, z.core.$ZodTypeInternals<ReferenceEpoch, string>>>;
		jitter: z.ZodMiniOptional<z.ZodMiniPipe<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">, z.ZodMiniTransform<number & z.$brand<"u53"> | undefined, number & z.$brand<"u53">>>>;
		label: z.ZodMiniOptional<z.ZodMiniString<string>>;
		layout: z.ZodMiniOptional<z.ZodMiniString<string>>;
		numberOfChannels: z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">;
		sampleRate: z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">;
		track: z.ZodMiniOptional<z.ZodMiniString<string>>;
		warmup: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>;
	}, z.core.$strip>;
	export const AudioSchema: z.ZodMiniUnion<readonly [z.ZodMiniObject<{
		renditions: z.ZodMiniRecord<z.ZodMiniString<string>, z.ZodMiniObject<{
			bitrate: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<...>, "u53", "out">>;
			broadcast: z.ZodMiniOptional<z.ZodMiniType<RelativeBroadcast, string, z.core.$ZodTypeInternals<..., ...>>>;
			codec: z.ZodMiniString<string>;
			container: z.ZodMiniDefault<z.ZodMiniUnion<readonly ...>>;
			delay: z.ZodMiniOptional<z.ZodMiniPipe<z.core.$ZodBranded<..., ..., ...>, z.ZodMiniTransform<..., ...>>>;
			description: z.ZodMiniOptional<z.ZodMiniString<string>>;
			enabled: z.ZodMiniOptional<z.ZodMiniBoolean<boolean>>;
			epoch: z.ZodMiniOptional<z.ZodMiniType<ReferenceEpoch, string, z.core.$ZodTypeInternals<..., ...>>>;
			jitter: z.ZodMiniOptional<z.ZodMiniPipe<z.core.$ZodBranded<..., ..., ...>, z.ZodMiniTransform<..., ...>>>;
			label: z.ZodMiniOptional<z.ZodMiniString<string>>;
			layout: z.ZodMiniOptional<z.ZodMiniString<string>>;
			numberOfChannels: z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">;
			sampleRate: z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">;
			track: z.ZodMiniOptional<z.ZodMiniString<string>>;
			warmup: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<...>, "u53", "out">>;
		}, z.core.$strip>>;
	}, z.core.$strip>, z.ZodMiniPipe<z.ZodMiniObject<{
		config: z.ZodMiniObject<{
			bitrate: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<...>, "u53", "out">>;
			broadcast: z.ZodMiniOptional<z.ZodMiniType<RelativeBroadcast, string, z.core.$ZodTypeInternals<..., ...>>>;
			codec: z.ZodMiniString<string>;
			container: z.ZodMiniDefault<z.ZodMiniUnion<readonly ...>>;
			delay: z.ZodMiniOptional<z.ZodMiniPipe<z.core.$ZodBranded<..., ..., ...>, z.ZodMiniTransform<..., ...>>>;
			description: z.ZodMiniOptional<z.ZodMiniString<string>>;
			enabled: z.ZodMiniOptional<z.ZodMiniBoolean<boolean>>;
			epoch: z.ZodMiniOptional<z.ZodMiniType<ReferenceEpoch, string, z.core.$ZodTypeInternals<..., ...>>>;
			jitter: z.ZodMiniOptional<z.ZodMiniPipe<z.core.$ZodBranded<..., ..., ...>, z.ZodMiniTransform<..., ...>>>;
			label: z.ZodMiniOptional<z.ZodMiniString<string>>;
			layout: z.ZodMiniOptional<z.ZodMiniString<string>>;
			numberOfChannels: z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">;
			sampleRate: z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">;
			track: z.ZodMiniOptional<z.ZodMiniString<string>>;
			warmup: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<...>, "u53", "out">>;
		}, z.core.$strip>;
		track: z.ZodMiniObject<{
			name: z.ZodMiniString<string>;
		}, z.core.$strip>;
	}, z.core.$strip>, z.ZodMiniTransform<{
		renditions: {
			[x: string]: {
				bitrate?: ... & ...;
				broadcast?: RelativeBroadcast;
				codec: string;
				container: {
					kind: "legacy";
				} | {
					init: string;
					kind: "cmaf";
				} | {
					kind: "loc";
				} | {
					kind: "unknown";
					raw: {
						[x: ...]: ...;
						kind: ...;
					};
				};
				delay?: ... & ...;
				description?: string;
				enabled?: boolean;
				epoch?: ReferenceEpoch;
				jitter?: ... & ...;
				label?: string;
				layout?: string;
				numberOfChannels: number & z.$brand<"u53">;
				sampleRate: number & z.$brand<"u53">;
				track?: string;
				warmup?: ... & ...;
			};
		};
	}, {
		config: {
			bitrate?: number & z.$brand<...>;
			broadcast?: RelativeBroadcast;
			codec: string;
			container: {
				kind: "legacy";
			} | {
				init: string;
				kind: "cmaf";
			} | {
				kind: "loc";
			} | {
				kind: "unknown";
				raw: {
					[x: string]: unknown;
					kind: string;
				};
			};
			delay?: number & z.$brand<...>;
			description?: string;
			enabled?: boolean;
			epoch?: ReferenceEpoch;
			jitter?: number & z.$brand<...>;
			label?: string;
			layout?: string;
			numberOfChannels: number & z.$brand<"u53">;
			sampleRate: number & z.$brand<"u53">;
			track?: string;
			warmup?: number & z.$brand<...>;
		};
		track: {
			name: string;
		};
	}>>]>;
	export function ranked(renditions: Record<string, AudioConfig>): [string, {
		bitrate?: number & $brand<"u53">;
		broadcast?: RelativeBroadcast;
		codec: string;
		container: {
			kind: "unknown";
			raw: {
				[x: string]: unknown;
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
	}][];
}

export type Audio = audio.Audio;

export import AudioConfig = Audio.AudioConfig;

export import AudioConfigSchema = Audio.AudioConfigSchema;

export import AudioSchema = Audio.AudioSchema;

export type Binary = z.infer<typeof BinarySchema>;

export type BinaryConfig = z.infer<typeof BinaryConfigSchema>;

export const BinaryConfigSchema: z.ZodMiniObject<{
	bitrate: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>;
	broadcast: z.ZodMiniOptional<z.ZodMiniType<RelativeBroadcast, string, z.core.$ZodTypeInternals<RelativeBroadcast, string>>>;
	compression: z.ZodMiniOptional<z.ZodMiniString<string>>;
	delay: z.ZodMiniOptional<z.ZodMiniPipe<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">, z.ZodMiniTransform<number & z.$brand<"u53"> | undefined, number & z.$brand<"u53">>>>;
	epoch: z.ZodMiniOptional<z.ZodMiniType<ReferenceEpoch, string, z.core.$ZodTypeInternals<ReferenceEpoch, string>>>;
	jitter: z.ZodMiniOptional<z.ZodMiniPipe<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">, z.ZodMiniTransform<number & z.$brand<"u53"> | undefined, number & z.$brand<"u53">>>>;
	mime: z.ZodMiniOptional<z.ZodMiniString<string>>;
	mode: z.ZodMiniString<string>;
	track: z.ZodMiniOptional<z.ZodMiniString<string>>;
}, z.core.$loose>;

export const BinarySchema: z.ZodMiniObject<{
	tracks: z.ZodMiniRecord<z.ZodMiniString<string>, z.ZodMiniObject<{
		bitrate: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>;
		broadcast: z.ZodMiniOptional<z.ZodMiniType<RelativeBroadcast, string, z.core.$ZodTypeInternals<RelativeBroadcast, string>>>;
		compression: z.ZodMiniOptional<z.ZodMiniString<string>>;
		delay: z.ZodMiniOptional<z.ZodMiniPipe<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">, z.ZodMiniTransform<... & ... | undefined, number & z.$brand<"u53">>>>;
		epoch: z.ZodMiniOptional<z.ZodMiniType<ReferenceEpoch, string, z.core.$ZodTypeInternals<ReferenceEpoch, string>>>;
		jitter: z.ZodMiniOptional<z.ZodMiniPipe<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">, z.ZodMiniTransform<... & ... | undefined, number & z.$brand<"u53">>>>;
		mime: z.ZodMiniOptional<z.ZodMiniString<string>>;
		mode: z.ZodMiniString<string>;
		track: z.ZodMiniOptional<z.ZodMiniString<string>>;
	}, z.core.$loose>>;
}, z.core.$strip>;

export type Clock = z.infer<typeof ClockSchema>;

export const ClockSchema: z.ZodMiniObject<{
	timescale: z.ZodMiniDefault<z.ZodMiniNumber<number>>;
	wall: z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">;
}, z.core.$strip>;

export type CmafContainer = Extract<Container, {
	kind: "cmaf";
}>;

export import Color = Video.Color;

export import ColorSchema = Video.ColorSchema;

export type Compression = z.infer<typeof CompressionSchema>;

export const CompressionSchema: z.ZodMiniString<string>;

export type Container = z.infer<typeof ContainerSchema>;

export const ContainerSchema: z.ZodMiniDefault<z.ZodMiniUnion<readonly [z.ZodMiniDiscriminatedUnion<[z.ZodMiniObject<{
	kind: z.ZodMiniLiteral<"legacy">;
}, z.core.$strip>, z.ZodMiniObject<{
	init: z.ZodMiniBase64;
	kind: z.ZodMiniLiteral<"cmaf">;
}, z.core.$strip>, z.ZodMiniObject<{
	kind: z.ZodMiniLiteral<"loc">;
}, z.core.$strip>], "kind">, z.ZodMiniPipe<z.ZodMiniObject<{
	kind: z.ZodMiniString<string>;
}, z.core.$loose>, z.ZodMiniTransform<{
	kind: "unknown";
	raw: {
		[x: string]: unknown;
		kind: string;
	};
}, {
	[x: string]: unknown;
	kind: string;
}>>]>>;

export const DEFAULT_FORMAT: Format;

export class DuplicateRendition extends Error {
	constructor(id: string);
	readonly id: string;
}

export type Echo = z.infer<typeof EchoSchema>;

export const EchoSchema: z.ZodMiniObject<{
	path: z.ZodMiniType<RelativeBroadcast, string, z.core.$ZodTypeInternals<RelativeBroadcast, string>>;
}, z.core.$strip>;

export const EpochSchema: z.ZodMiniType<ReferenceEpoch, string>;

export class EscapingBroadcast extends Error {
	constructor(broadcast: RelativeBroadcast);
	readonly broadcast: RelativeBroadcast;
}

export const FORMATS: readonly ["hang", "msf"];

export type Format = typeof FORMATS[number];

export type Hex = z.infer<typeof hexSchema>;

export type Json = z.infer<typeof JsonSchema>;

export type JsonConfig = z.infer<typeof JsonConfigSchema>;

export const JsonConfigSchema: z.ZodMiniObject<{
	bitrate: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>;
	broadcast: z.ZodMiniOptional<z.ZodMiniType<RelativeBroadcast, string, z.core.$ZodTypeInternals<RelativeBroadcast, string>>>;
	compression: z.ZodMiniOptional<z.ZodMiniString<string>>;
	delay: z.ZodMiniOptional<z.ZodMiniPipe<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">, z.ZodMiniTransform<number & z.$brand<"u53"> | undefined, number & z.$brand<"u53">>>>;
	epoch: z.ZodMiniOptional<z.ZodMiniType<ReferenceEpoch, string, z.core.$ZodTypeInternals<ReferenceEpoch, string>>>;
	jitter: z.ZodMiniOptional<z.ZodMiniPipe<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">, z.ZodMiniTransform<number & z.$brand<"u53"> | undefined, number & z.$brand<"u53">>>>;
	mode: z.ZodMiniString<string>;
	schema: z.ZodMiniOptional<z.ZodMiniString<string>>;
	track: z.ZodMiniOptional<z.ZodMiniString<string>>;
}, z.core.$loose>;

export const JsonSchema: z.ZodMiniObject<{
	tracks: z.ZodMiniRecord<z.ZodMiniString<string>, z.ZodMiniObject<{
		bitrate: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>;
		broadcast: z.ZodMiniOptional<z.ZodMiniType<RelativeBroadcast, string, z.core.$ZodTypeInternals<RelativeBroadcast, string>>>;
		compression: z.ZodMiniOptional<z.ZodMiniString<string>>;
		delay: z.ZodMiniOptional<z.ZodMiniPipe<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">, z.ZodMiniTransform<... & ... | undefined, number & z.$brand<"u53">>>>;
		epoch: z.ZodMiniOptional<z.ZodMiniType<ReferenceEpoch, string, z.core.$ZodTypeInternals<ReferenceEpoch, string>>>;
		jitter: z.ZodMiniOptional<z.ZodMiniPipe<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">, z.ZodMiniTransform<... & ... | undefined, number & z.$brand<"u53">>>>;
		mode: z.ZodMiniString<string>;
		schema: z.ZodMiniOptional<z.ZodMiniString<string>>;
		track: z.ZodMiniOptional<z.ZodMiniString<string>>;
	}, z.core.$loose>>;
}, z.core.$strip>;

export type KnownCompression = "deflate";

export type KnownMode = "snapshot" | "stream";

export const MOQ_EPOCH_UNIX_MILLIS: 1577836800000;

export type Mode = z.infer<typeof ModeSchema>;

export const ModeSchema: z.ZodMiniString<string>;

export const PRIORITY: {
	readonly audio: 80;
	readonly catalog: 100;
	readonly text: 90;
	readonly video: 60;
};

export import Projection = Video.Projection;

export import ProjectionSchema = Video.ProjectionSchema;

export type ReferenceEpoch = string & {
	readonly _brand: "epoch";
};

export type RelativeBroadcast = string & {
	__brand: "Relative";
};

export const RelativeBroadcastSchema: z.ZodMiniType<RelativeBroadcast, string>;

export type Root = z.infer<typeof RootSchema>;

export const RootSchema: z.ZodMiniObject<{
	archive: z.ZodMiniCatch<z.ZodMiniOptional<z.ZodMiniObject<{
		replay: z.ZodMiniOptional<z.ZodMiniType<RelativeBroadcast, string, z.core.$ZodTypeInternals<RelativeBroadcast, string>>>;
		replayEpoch: z.ZodMiniOptional<z.ZodMiniType<ReferenceEpoch, string, z.core.$ZodTypeInternals<ReferenceEpoch, string>>>;
		store: z.ZodMiniOptional<z.ZodMiniURL>;
		timelines: z.ZodMiniRecord<z.ZodMiniString<string>, z.ZodMiniObject<{
			duration: z.ZodMiniOptional<z.core.$ZodBranded<..., ..., ...>>;
			track: z.ZodMiniString<string>;
		}, z.core.$strip>>;
		timescale: z.ZodMiniDefault<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>;
		version: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>;
	}, z.core.$strip>>>;
	audio: z.ZodMiniOptional<z.ZodMiniUnion<readonly [z.ZodMiniObject<{
		renditions: z.ZodMiniRecord<z.ZodMiniString<string>, z.ZodMiniObject<{
			bitrate: ...;
			broadcast: ...;
			codec: ...;
			container: ...;
			delay: ...;
			description: ...;
			enabled: ...;
			epoch: ...;
			jitter: ...;
			label: ...;
			layout: ...;
			numberOfChannels: ...;
			sampleRate: ...;
			track: ...;
			warmup: ...;
		}, z.core.$strip>>;
	}, z.core.$strip>, z.ZodMiniPipe<z.ZodMiniObject<{
		config: z.ZodMiniObject<{
			bitrate: ...;
			broadcast: ...;
			codec: ...;
			container: ...;
			delay: ...;
			description: ...;
			enabled: ...;
			epoch: ...;
			jitter: ...;
			label: ...;
			layout: ...;
			numberOfChannels: ...;
			sampleRate: ...;
			track: ...;
			warmup: ...;
		}, z.core.$strip>;
		track: z.ZodMiniObject<{
			name: ...;
		}, z.core.$strip>;
	}, z.core.$strip>, z.ZodMiniTransform<{
		renditions: {
			[x: string]: {
				bitrate?: ...;
				broadcast?: ...;
				codec: ...;
				container: ...;
				delay?: ...;
				description?: ...;
				enabled?: ...;
				epoch?: ...;
				jitter?: ...;
				label?: ...;
				layout?: ...;
				numberOfChannels: ...;
				sampleRate: ...;
				track?: ...;
				warmup?: ...;
			};
		};
	}, {
		config: {
			bitrate?: ... | ...;
			broadcast?: ... | ...;
			codec: string;
			container: ... | ... | ... | ...;
			delay?: ... | ...;
			description?: ... | ...;
			enabled?: ... | ...;
			epoch?: ... | ...;
			jitter?: ... | ...;
			label?: ... | ...;
			layout?: ... | ...;
			numberOfChannels: ... & ...;
			sampleRate: ... & ...;
			track?: ... | ...;
			warmup?: ... | ...;
		};
		track: {
			name: string;
		};
	}>>]>>;
	binary: z.ZodMiniOptional<z.ZodMiniUnion<readonly [z.ZodMiniObject<{
		tracks: z.ZodMiniRecord<z.ZodMiniString<string>, z.ZodMiniObject<{
			bitrate: ...;
			broadcast: ...;
			compression: ...;
			delay: ...;
			epoch: ...;
			jitter: ...;
			mime: ...;
			mode: ...;
			track: ...;
		}, z.core.$loose>>;
	}, z.core.$strip>, z.ZodMiniPipe<z.ZodMiniCustom<unknown, unknown>, z.ZodMiniTransform<undefined, unknown>>]>>;
	clock: z.ZodMiniOptional<z.ZodMiniObject<{
		timescale: z.ZodMiniDefault<z.ZodMiniNumber<number>>;
		wall: z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">;
	}, z.core.$strip>>;
	echo: z.ZodMiniOptional<z.ZodMiniUnion<readonly [z.ZodMiniObject<{
		path: z.ZodMiniType<RelativeBroadcast, string, z.core.$ZodTypeInternals<RelativeBroadcast, string>>;
	}, z.core.$strip>, z.ZodMiniPipe<z.ZodMiniCustom<unknown, unknown>, z.ZodMiniTransform<undefined, unknown>>]>>;
	json: z.ZodMiniOptional<z.ZodMiniUnion<readonly [z.ZodMiniObject<{
		tracks: z.ZodMiniRecord<z.ZodMiniString<string>, z.ZodMiniObject<{
			bitrate: ...;
			broadcast: ...;
			compression: ...;
			delay: ...;
			epoch: ...;
			jitter: ...;
			mode: ...;
			schema: ...;
			track: ...;
		}, z.core.$loose>>;
	}, z.core.$strip>, z.ZodMiniPipe<z.ZodMiniCustom<unknown, unknown>, z.ZodMiniTransform<undefined, unknown>>]>>;
	stats: z.ZodMiniOptional<z.ZodMiniUnion<readonly [z.ZodMiniObject<{
		track: z.ZodMiniString<string>;
	}, z.core.$strip>, z.ZodMiniPipe<z.ZodMiniCustom<unknown, unknown>, z.ZodMiniTransform<undefined, unknown>>]>>;
	text: z.ZodMiniOptional<z.ZodMiniUnion<readonly [z.ZodMiniObject<{
		renditions: z.ZodMiniRecord<z.ZodMiniString<string>, z.ZodMiniObject<{
			broadcast: ...;
			container: ...;
			delay: ...;
			epoch: ...;
			format: ...;
			jitter: ...;
			label: ...;
			lang: ...;
			role: ...;
			track: ...;
		}, z.core.$strip>>;
	}, z.core.$strip>, z.ZodMiniPipe<z.ZodMiniCustom<unknown, unknown>, z.ZodMiniTransform<undefined, unknown>>]>>;
	version: z.ZodMiniOptional<z.ZodMiniNumber<number>>;
	video: z.ZodMiniOptional<z.ZodMiniUnion<readonly [z.ZodMiniObject<{
		display: z.ZodMiniOptional<z.ZodMiniObject<{
			height: ...;
			width: ...;
		}, z.core.$strip>>;
		flip: z.ZodMiniOptional<z.ZodMiniBoolean<boolean>>;
		renditions: z.ZodMiniRecord<z.ZodMiniString<string>, z.ZodMiniObject<{
			bitrate: ...;
			broadcast: ...;
			codec: ...;
			codedHeight: ...;
			codedWidth: ...;
			color: ...;
			container: ...;
			delay: ...;
			description: ...;
			displayAspectHeight: ...;
			displayAspectWidth: ...;
			enabled: ...;
			epoch: ...;
			framerate: ...;
			interlaced: ...;
			jitter: ...;
			label: ...;
			optimizeForLatency: ...;
			preference: ...;
			projection: ...;
			reorder: ...;
			stereo: ...;
			track: ...;
			warmup: ...;
		}, z.core.$strip>>;
		rotation: z.ZodMiniOptional<z.ZodMiniNumber<number>>;
	}, z.core.$strip>, z.ZodMiniPipe<z.ZodMiniArray<z.ZodMiniObject<{
		config: z.ZodMiniObject<..., ...>;
		track: z.ZodMiniObject<..., ...>;
	}, z.core.$strip>>, z.ZodMiniTransform<{
		display: {
			height: ...;
			width: ...;
		} | undefined;
		flip: undefined;
		renditions: {
			[k: string]: {
				bitrate?: ...;
				broadcast?: ...;
				codec: ...;
				codedHeight?: ...;
				codedWidth?: ...;
				color?: ...;
				container: ...;
				delay?: ...;
				description?: ...;
				displayAspectHeight?: ...;
				displayAspectWidth?: ...;
				enabled?: ...;
				epoch?: ...;
				framerate?: ...;
				interlaced?: ...;
				jitter?: ...;
				label?: ...;
				optimizeForLatency?: ...;
				preference?: ...;
				projection?: ...;
				reorder?: ...;
				stereo?: ...;
				track?: ...;
				warmup?: ...;
			};
		};
		rotation: undefined;
	}, {
		config: {
			bitrate?: ...;
			broadcast?: ...;
			codec: ...;
			codedHeight?: ...;
			codedWidth?: ...;
			color?: ...;
			container: ...;
			delay?: ...;
			description?: ...;
			displayAspectHeight?: ...;
			displayAspectWidth?: ...;
			enabled?: ...;
			epoch?: ...;
			framerate?: ...;
			interlaced?: ...;
			jitter?: ...;
			label?: ...;
			optimizeForLatency?: ...;
			preference?: ...;
			projection?: ...;
			reorder?: ...;
			stereo?: ...;
			track?: ...;
			warmup?: ...;
		};
		track: {
			name: ...;
		};
	}[]>>]>>;
}, z.core.$loose>;

export type Stats = z.infer<typeof StatsSchema>;

export const StatsSchema: z.ZodMiniObject<{
	track: z.ZodMiniString<string>;
}, z.core.$strip>;

export const TRACK: "catalog.json";

export const TRACK_COMPRESSED: "catalog.json.z";

export type Text = z.infer<typeof TextSchema>;

export type TextConfig = z.infer<typeof TextConfigSchema>;

export const TextConfigSchema: z.ZodMiniObject<{
	broadcast: z.ZodMiniOptional<z.ZodMiniType<RelativeBroadcast, string, z.core.$ZodTypeInternals<RelativeBroadcast, string>>>;
	container: z.ZodMiniDefault<z.ZodMiniUnion<readonly [z.ZodMiniDiscriminatedUnion<[z.ZodMiniObject<{
		kind: z.ZodMiniLiteral<...>;
	}, z.core.$strip>, z.ZodMiniObject<{
		init: z.ZodMiniBase64;
		kind: z.ZodMiniLiteral<...>;
	}, z.core.$strip>, z.ZodMiniObject<{
		kind: z.ZodMiniLiteral<...>;
	}, z.core.$strip>], "kind">, z.ZodMiniPipe<z.ZodMiniObject<{
		kind: z.ZodMiniString<string>;
	}, z.core.$loose>, z.ZodMiniTransform<{
		kind: "unknown";
		raw: {
			[x: string]: unknown;
			kind: string;
		};
	}, {
		[x: string]: unknown;
		kind: string;
	}>>]>>;
	delay: z.ZodMiniOptional<z.ZodMiniPipe<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">, z.ZodMiniTransform<number & z.$brand<"u53"> | undefined, number & z.$brand<"u53">>>>;
	epoch: z.ZodMiniOptional<z.ZodMiniType<ReferenceEpoch, string, z.core.$ZodTypeInternals<ReferenceEpoch, string>>>;
	format: z.ZodMiniString<string>;
	jitter: z.ZodMiniOptional<z.ZodMiniPipe<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">, z.ZodMiniTransform<number & z.$brand<"u53"> | undefined, number & z.$brand<"u53">>>>;
	label: z.ZodMiniOptional<z.ZodMiniString<string>>;
	lang: z.ZodMiniOptional<z.ZodMiniString<string>>;
	role: z.ZodMiniDefault<z.ZodMiniString<string>>;
	track: z.ZodMiniOptional<z.ZodMiniString<string>>;
}, z.core.$strip>;

export const TextFormatSchema: z.ZodMiniString<string>;

export type TextRole = z.infer<typeof TextRoleSchema>;

export const TextRoleSchema: z.ZodMiniString<string>;

export const TextSchema: z.ZodMiniObject<{
	renditions: z.ZodMiniRecord<z.ZodMiniString<string>, z.ZodMiniObject<{
		broadcast: z.ZodMiniOptional<z.ZodMiniType<RelativeBroadcast, string, z.core.$ZodTypeInternals<RelativeBroadcast, string>>>;
		container: z.ZodMiniDefault<z.ZodMiniUnion<readonly [z.ZodMiniDiscriminatedUnion<[..., ..., ...], "kind">, z.ZodMiniPipe<z.ZodMiniObject<..., ...>, z.ZodMiniTransform<..., ...>>]>>;
		delay: z.ZodMiniOptional<z.ZodMiniPipe<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">, z.ZodMiniTransform<... & ... | undefined, number & z.$brand<"u53">>>>;
		epoch: z.ZodMiniOptional<z.ZodMiniType<ReferenceEpoch, string, z.core.$ZodTypeInternals<ReferenceEpoch, string>>>;
		format: z.ZodMiniString<string>;
		jitter: z.ZodMiniOptional<z.ZodMiniPipe<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">, z.ZodMiniTransform<... & ... | undefined, number & z.$brand<"u53">>>>;
		label: z.ZodMiniOptional<z.ZodMiniString<string>>;
		lang: z.ZodMiniOptional<z.ZodMiniString<string>>;
		role: z.ZodMiniDefault<z.ZodMiniString<string>>;
		track: z.ZodMiniOptional<z.ZodMiniString<string>>;
	}, z.core.$strip>>;
}, z.core.$strip>;

export type Track = z.infer<typeof TrackSchema>;

export const TrackSchema: z.ZodMiniObject<{
	name: z.ZodMiniString<string>;
}, z.core.$strip>;

export type U53 = z.infer<typeof u53Schema>;

export type U8 = z.infer<typeof u8Schema>;

export const UnknownContainerSchema: z.ZodMiniPipe<z.ZodMiniObject<{
	kind: z.ZodMiniString<string>;
}, z.core.$loose>, z.ZodMiniTransform<{
	kind: "unknown";
	raw: {
		[x: string]: unknown;
		kind: string;
	};
}, {
	[x: string]: unknown;
	kind: string;
}>>;

export class UnsupportedVersion extends Error {
	constructor(version: number);
	readonly version: number;
}

export const VERSION: 4;

export namespace Video {
	export type Color = z.infer<typeof ColorSchema>;
	export const ColorSchema: z.ZodMiniObject<{
		contentLight: z.ZodMiniOptional<z.ZodMiniObject<{
			maxCll: z.ZodMiniNumber<number>;
			maxFall: z.ZodMiniNumber<number>;
		}, z.core.$strip>>;
		mastering: z.ZodMiniOptional<z.ZodMiniObject<{
			blue: z.ZodMiniObject<{
				x: z.ZodMiniNumber<number>;
				y: z.ZodMiniNumber<number>;
			}, z.core.$strip>;
			green: z.ZodMiniObject<{
				x: z.ZodMiniNumber<number>;
				y: z.ZodMiniNumber<number>;
			}, z.core.$strip>;
			maxLuminance: z.ZodMiniNumber<number>;
			minLuminance: z.ZodMiniNumber<number>;
			red: z.ZodMiniObject<{
				x: z.ZodMiniNumber<number>;
				y: z.ZodMiniNumber<number>;
			}, z.core.$strip>;
			white: z.ZodMiniObject<{
				x: z.ZodMiniNumber<number>;
				y: z.ZodMiniNumber<number>;
			}, z.core.$strip>;
		}, z.core.$strip>>;
		matrix: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u8", "out">>;
		primaries: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u8", "out">>;
		range: z.ZodMiniOptional<z.ZodMiniEnum<{
			full: "full";
			limited: "limited";
		}>>;
		transfer: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u8", "out">>;
	}, z.core.$strip>;
	export type Projection = z.infer<typeof ProjectionSchema>;
	export const ProjectionSchema: z.ZodMiniUnion<readonly [z.ZodMiniDiscriminatedUnion<[z.ZodMiniObject<{
		kind: z.ZodMiniLiteral<"rectilinear">;
	}, z.core.$strip>, z.ZodMiniObject<{
		fov: z.ZodMiniOptional<z.ZodMiniNumber<number>>;
		kind: z.ZodMiniLiteral<"equirectangular">;
	}, z.core.$strip>], "kind">, z.ZodMiniPipe<z.ZodMiniObject<{
		kind: z.ZodMiniString<string>;
	}, z.core.$loose>, z.ZodMiniTransform<{
		kind: "unknown";
		raw: {
			[x: string]: unknown;
			kind: string;
		};
	}, {
		[x: string]: unknown;
		kind: string;
	}>>]>;
	export type Video = z.infer<typeof VideoSchema>;
	export type VideoConfig = z.infer<typeof VideoConfigSchema>;
	export const VideoConfigSchema: z.ZodMiniObject<{
		bitrate: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>;
		broadcast: z.ZodMiniOptional<z.ZodMiniType<RelativeBroadcast, string, z.core.$ZodTypeInternals<RelativeBroadcast, string>>>;
		codec: z.ZodMiniString<string>;
		codedHeight: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>;
		codedWidth: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>;
		color: z.ZodMiniOptional<z.ZodMiniObject<{
			contentLight: z.ZodMiniOptional<z.ZodMiniObject<{
				maxCll: z.ZodMiniNumber<number>;
				maxFall: z.ZodMiniNumber<number>;
			}, z.core.$strip>>;
			mastering: z.ZodMiniOptional<z.ZodMiniObject<{
				blue: z.ZodMiniObject<{
					x: z.ZodMiniNumber<...>;
					y: z.ZodMiniNumber<...>;
				}, z.core.$strip>;
				green: z.ZodMiniObject<{
					x: z.ZodMiniNumber<...>;
					y: z.ZodMiniNumber<...>;
				}, z.core.$strip>;
				maxLuminance: z.ZodMiniNumber<number>;
				minLuminance: z.ZodMiniNumber<number>;
				red: z.ZodMiniObject<{
					x: z.ZodMiniNumber<...>;
					y: z.ZodMiniNumber<...>;
				}, z.core.$strip>;
				white: z.ZodMiniObject<{
					x: z.ZodMiniNumber<...>;
					y: z.ZodMiniNumber<...>;
				}, z.core.$strip>;
			}, z.core.$strip>>;
			matrix: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u8", "out">>;
			primaries: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u8", "out">>;
			range: z.ZodMiniOptional<z.ZodMiniEnum<{
				full: "full";
				limited: "limited";
			}>>;
			transfer: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u8", "out">>;
		}, z.core.$strip>>;
		container: z.ZodMiniDefault<z.ZodMiniUnion<readonly [z.ZodMiniDiscriminatedUnion<[z.ZodMiniObject<{
			kind: z.ZodMiniLiteral<...>;
		}, z.core.$strip>, z.ZodMiniObject<{
			init: z.ZodMiniBase64;
			kind: z.ZodMiniLiteral<...>;
		}, z.core.$strip>, z.ZodMiniObject<{
			kind: z.ZodMiniLiteral<...>;
		}, z.core.$strip>], "kind">, z.ZodMiniPipe<z.ZodMiniObject<{
			kind: z.ZodMiniString<string>;
		}, z.core.$loose>, z.ZodMiniTransform<{
			kind: "unknown";
			raw: {
				[x: string]: unknown;
				kind: string;
			};
		}, {
			[x: string]: unknown;
			kind: string;
		}>>]>>;
		delay: z.ZodMiniOptional<z.ZodMiniPipe<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">, z.ZodMiniTransform<number & z.$brand<"u53"> | undefined, number & z.$brand<"u53">>>>;
		description: z.ZodMiniOptional<z.ZodMiniString<string>>;
		displayAspectHeight: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>;
		displayAspectWidth: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>;
		enabled: z.ZodMiniOptional<z.ZodMiniBoolean<boolean>>;
		epoch: z.ZodMiniOptional<z.ZodMiniType<ReferenceEpoch, string, z.core.$ZodTypeInternals<ReferenceEpoch, string>>>;
		framerate: z.ZodMiniOptional<z.ZodMiniNumber<number>>;
		interlaced: z.ZodMiniOptional<z.ZodMiniBoolean<boolean>>;
		jitter: z.ZodMiniOptional<z.ZodMiniPipe<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">, z.ZodMiniTransform<number & z.$brand<"u53"> | undefined, number & z.$brand<"u53">>>>;
		label: z.ZodMiniOptional<z.ZodMiniString<string>>;
		optimizeForLatency: z.ZodMiniOptional<z.ZodMiniBoolean<boolean>>;
		preference: z.ZodMiniOptional<z.ZodMiniInt32>;
		projection: z.ZodMiniOptional<z.ZodMiniUnion<readonly [z.ZodMiniDiscriminatedUnion<[z.ZodMiniObject<{
			kind: z.ZodMiniLiteral<...>;
		}, z.core.$strip>, z.ZodMiniObject<{
			fov: z.ZodMiniOptional<...>;
			kind: z.ZodMiniLiteral<...>;
		}, z.core.$strip>], "kind">, z.ZodMiniPipe<z.ZodMiniObject<{
			kind: z.ZodMiniString<string>;
		}, z.core.$loose>, z.ZodMiniTransform<{
			kind: "unknown";
			raw: {
				[x: string]: unknown;
				kind: string;
			};
		}, {
			[x: string]: unknown;
			kind: string;
		}>>]>>;
		reorder: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u8", "out">>;
		stereo: z.ZodMiniOptional<z.ZodMiniString<string>>;
		track: z.ZodMiniOptional<z.ZodMiniString<string>>;
		warmup: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>;
	}, z.core.$strip>;
	export const VideoSchema: z.ZodMiniUnion<readonly [z.ZodMiniObject<{
		display: z.ZodMiniOptional<z.ZodMiniObject<{
			height: z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">;
			width: z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">;
		}, z.core.$strip>>;
		flip: z.ZodMiniOptional<z.ZodMiniBoolean<boolean>>;
		renditions: z.ZodMiniRecord<z.ZodMiniString<string>, z.ZodMiniObject<{
			bitrate: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<...>, "u53", "out">>;
			broadcast: z.ZodMiniOptional<z.ZodMiniType<RelativeBroadcast, string, z.core.$ZodTypeInternals<..., ...>>>;
			codec: z.ZodMiniString<string>;
			codedHeight: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<...>, "u53", "out">>;
			codedWidth: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<...>, "u53", "out">>;
			color: z.ZodMiniOptional<z.ZodMiniObject<{
				contentLight: ...;
				mastering: ...;
				matrix: ...;
				primaries: ...;
				range: ...;
				transfer: ...;
			}, z.core.$strip>>;
			container: z.ZodMiniDefault<z.ZodMiniUnion<readonly ...>>;
			delay: z.ZodMiniOptional<z.ZodMiniPipe<z.core.$ZodBranded<..., ..., ...>, z.ZodMiniTransform<..., ...>>>;
			description: z.ZodMiniOptional<z.ZodMiniString<string>>;
			displayAspectHeight: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<...>, "u53", "out">>;
			displayAspectWidth: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<...>, "u53", "out">>;
			enabled: z.ZodMiniOptional<z.ZodMiniBoolean<boolean>>;
			epoch: z.ZodMiniOptional<z.ZodMiniType<ReferenceEpoch, string, z.core.$ZodTypeInternals<..., ...>>>;
			framerate: z.ZodMiniOptional<z.ZodMiniNumber<number>>;
			interlaced: z.ZodMiniOptional<z.ZodMiniBoolean<boolean>>;
			jitter: z.ZodMiniOptional<z.ZodMiniPipe<z.core.$ZodBranded<..., ..., ...>, z.ZodMiniTransform<..., ...>>>;
			label: z.ZodMiniOptional<z.ZodMiniString<string>>;
			optimizeForLatency: z.ZodMiniOptional<z.ZodMiniBoolean<boolean>>;
			preference: z.ZodMiniOptional<z.ZodMiniInt32>;
			projection: z.ZodMiniOptional<z.ZodMiniUnion<readonly ...>>;
			reorder: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<...>, "u8", "out">>;
			stereo: z.ZodMiniOptional<z.ZodMiniString<string>>;
			track: z.ZodMiniOptional<z.ZodMiniString<string>>;
			warmup: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<...>, "u53", "out">>;
		}, z.core.$strip>>;
		rotation: z.ZodMiniOptional<z.ZodMiniNumber<number>>;
	}, z.core.$strip>, z.ZodMiniPipe<z.ZodMiniArray<z.ZodMiniObject<{
		config: z.ZodMiniObject<{
			bitrate: z.ZodMiniOptional<z.core.$ZodBranded<..., ..., ...>>;
			broadcast: z.ZodMiniOptional<z.ZodMiniType<..., ..., ...>>;
			codec: z.ZodMiniString<string>;
			codedHeight: z.ZodMiniOptional<z.core.$ZodBranded<..., ..., ...>>;
			codedWidth: z.ZodMiniOptional<z.core.$ZodBranded<..., ..., ...>>;
			color: z.ZodMiniOptional<z.ZodMiniObject<..., ...>>;
			container: z.ZodMiniDefault<z.ZodMiniUnion<...>>;
			delay: z.ZodMiniOptional<z.ZodMiniPipe<..., ...>>;
			description: z.ZodMiniOptional<z.ZodMiniString<...>>;
			displayAspectHeight: z.ZodMiniOptional<z.core.$ZodBranded<..., ..., ...>>;
			displayAspectWidth: z.ZodMiniOptional<z.core.$ZodBranded<..., ..., ...>>;
			enabled: z.ZodMiniOptional<z.ZodMiniBoolean<...>>;
			epoch: z.ZodMiniOptional<z.ZodMiniType<..., ..., ...>>;
			framerate: z.ZodMiniOptional<z.ZodMiniNumber<...>>;
			interlaced: z.ZodMiniOptional<z.ZodMiniBoolean<...>>;
			jitter: z.ZodMiniOptional<z.ZodMiniPipe<..., ...>>;
			label: z.ZodMiniOptional<z.ZodMiniString<...>>;
			optimizeForLatency: z.ZodMiniOptional<z.ZodMiniBoolean<...>>;
			preference: z.ZodMiniOptional<z.ZodMiniInt32>;
			projection: z.ZodMiniOptional<z.ZodMiniUnion<...>>;
			reorder: z.ZodMiniOptional<z.core.$ZodBranded<..., ..., ...>>;
			stereo: z.ZodMiniOptional<z.ZodMiniString<...>>;
			track: z.ZodMiniOptional<z.ZodMiniString<...>>;
			warmup: z.ZodMiniOptional<z.core.$ZodBranded<..., ..., ...>>;
		}, z.core.$strip>;
		track: z.ZodMiniObject<{
			name: z.ZodMiniString<string>;
		}, z.core.$strip>;
	}, z.core.$strip>>, z.ZodMiniTransform<{
		display: {
			height: number & z.$brand<"u53">;
			width: number & z.$brand<"u53">;
		} | undefined;
		flip: undefined;
		renditions: {
			[k: string]: {
				bitrate?: ... & ...;
				broadcast?: RelativeBroadcast;
				codec: string;
				codedHeight?: ... & ...;
				codedWidth?: ... & ...;
				color?: {
					contentLight?: ... | ...;
					mastering?: ... | ...;
					matrix?: ... | ...;
					primaries?: ... | ...;
					range?: ... | ... | ...;
					transfer?: ... | ...;
				};
				container: {
					kind: "legacy";
				} | {
					init: string;
					kind: "cmaf";
				} | {
					kind: "loc";
				} | {
					kind: "unknown";
					raw: {
						[x: ...]: ...;
						kind: ...;
					};
				};
				delay?: ... & ...;
				description?: string;
				displayAspectHeight?: ... & ...;
				displayAspectWidth?: ... & ...;
				enabled?: boolean;
				epoch?: ReferenceEpoch;
				framerate?: number;
				interlaced?: boolean;
				jitter?: ... & ...;
				label?: string;
				optimizeForLatency?: boolean;
				preference?: number;
				projection?: {
					kind: "rectilinear";
				} | {
					fov?: ... | ...;
					kind: "equirectangular";
				} | {
					kind: "unknown";
					raw: {
						[x: ...]: ...;
						kind: ...;
					};
				};
				reorder?: ... & ...;
				stereo?: string;
				track?: string;
				warmup?: ... & ...;
			};
		};
		rotation: undefined;
	}, {
		config: {
			bitrate?: ... & ...;
			broadcast?: RelativeBroadcast;
			codec: string;
			codedHeight?: ... & ...;
			codedWidth?: ... & ...;
			color?: {
				contentLight?: ... | ...;
				mastering?: ... | ...;
				matrix?: ... | ...;
				primaries?: ... | ...;
				range?: ... | ... | ...;
				transfer?: ... | ...;
			};
			container: {
				kind: "legacy";
			} | {
				init: string;
				kind: "cmaf";
			} | {
				kind: "loc";
			} | {
				kind: "unknown";
				raw: {
					[x: ...]: ...;
					kind: ...;
				};
			};
			delay?: ... & ...;
			description?: string;
			displayAspectHeight?: ... & ...;
			displayAspectWidth?: ... & ...;
			enabled?: boolean;
			epoch?: ReferenceEpoch;
			framerate?: number;
			interlaced?: boolean;
			jitter?: ... & ...;
			label?: string;
			optimizeForLatency?: boolean;
			preference?: number;
			projection?: {
				kind: "rectilinear";
			} | {
				fov?: ... | ...;
				kind: "equirectangular";
			} | {
				kind: "unknown";
				raw: {
					[x: ...]: ...;
					kind: ...;
				};
			};
			reorder?: ... & ...;
			stereo?: string;
			track?: string;
			warmup?: ... & ...;
		};
		track: {
			name: string;
		};
	}[]>>]>;
	export function ranked(renditions: Record<string, VideoConfig>): [string, {
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
			range?: "full" | "limited";
			transfer?: number & $brand<"u8">;
		};
		container: {
			kind: "unknown";
			raw: {
				[x: string]: unknown;
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
				[x: string]: unknown;
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
	}][];
}

export type Video = video.Video;

export import VideoConfig = Video.VideoConfig;

export import VideoConfigSchema = Video.VideoConfigSchema;

export import VideoSchema = Video.VideoSchema;

export function checkRenditions(root: {
	[key: string]: unknown;
	archive?: {
		replay?: RelativeBroadcast;
		replayEpoch?: ReferenceEpoch;
		store?: string;
		timelines: z.ZodMiniRecord<z.ZodMiniString<string>, z.ZodMiniObject<{
			duration: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>;
			track: z.ZodMiniString<string>;
		}, z.core.$strip>>;
		timescale: number & $brand<"u53">;
		version?: number & $brand<"u53">;
	};
	audio?: {
		renditions: {
			[x: string]: {
				bitrate?: number & $brand<"u53">;
				broadcast?: RelativeBroadcast;
				codec: string;
				container: {
					kind: "legacy";
				} | {
					init: string;
					kind: "cmaf";
				} | {
					kind: "loc";
				} | {
					kind: "unknown";
					raw: {
						[x: string]: unknown;
						kind: string;
					};
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
			};
		};
	} | {
		renditions: z.ZodMiniRecord<z.ZodMiniString<string>, z.ZodMiniObject<{
			bitrate: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>;
			broadcast: z.ZodMiniOptional<z.ZodMiniType<RelativeBroadcast, string, z.core.$ZodTypeInternals<RelativeBroadcast, string>>>;
			codec: z.ZodMiniString<string>;
			container: z.ZodMiniDefault<z.ZodMiniUnion<readonly [z.ZodMiniDiscriminatedUnion<..., ...>, z.ZodMiniPipe<..., ...>]>>;
			delay: z.ZodMiniOptional<z.ZodMiniPipe<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">, z.ZodMiniTransform<... | undefined, number & z.$brand<...>>>>;
			description: z.ZodMiniOptional<z.ZodMiniString<string>>;
			enabled: z.ZodMiniOptional<z.ZodMiniBoolean<boolean>>;
			epoch: z.ZodMiniOptional<z.ZodMiniType<ReferenceEpoch, string, z.core.$ZodTypeInternals<ReferenceEpoch, string>>>;
			jitter: z.ZodMiniOptional<z.ZodMiniPipe<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">, z.ZodMiniTransform<... | undefined, number & z.$brand<...>>>>;
			label: z.ZodMiniOptional<z.ZodMiniString<string>>;
			layout: z.ZodMiniOptional<z.ZodMiniString<string>>;
			numberOfChannels: z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">;
			sampleRate: z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">;
			track: z.ZodMiniOptional<z.ZodMiniString<string>>;
			warmup: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>;
		}, z.core.$strip>>;
	};
	binary?: {
		tracks: z.ZodMiniRecord<z.ZodMiniString<string>, z.ZodMiniObject<{
			bitrate: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>;
			broadcast: z.ZodMiniOptional<z.ZodMiniType<RelativeBroadcast, string, z.core.$ZodTypeInternals<RelativeBroadcast, string>>>;
			compression: z.ZodMiniOptional<z.ZodMiniString<string>>;
			delay: z.ZodMiniOptional<z.ZodMiniPipe<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">, z.ZodMiniTransform<... | undefined, number & z.$brand<...>>>>;
			epoch: z.ZodMiniOptional<z.ZodMiniType<ReferenceEpoch, string, z.core.$ZodTypeInternals<ReferenceEpoch, string>>>;
			jitter: z.ZodMiniOptional<z.ZodMiniPipe<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">, z.ZodMiniTransform<... | undefined, number & z.$brand<...>>>>;
			mime: z.ZodMiniOptional<z.ZodMiniString<string>>;
			mode: z.ZodMiniString<string>;
			track: z.ZodMiniOptional<z.ZodMiniString<string>>;
		}, z.core.$loose>>;
	};
	clock?: {
		timescale: number;
		wall: number & $brand<"u53">;
	};
	echo?: {
		path: RelativeBroadcast;
	};
	json?: {
		tracks: z.ZodMiniRecord<z.ZodMiniString<string>, z.ZodMiniObject<{
			bitrate: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>;
			broadcast: z.ZodMiniOptional<z.ZodMiniType<RelativeBroadcast, string, z.core.$ZodTypeInternals<RelativeBroadcast, string>>>;
			compression: z.ZodMiniOptional<z.ZodMiniString<string>>;
			delay: z.ZodMiniOptional<z.ZodMiniPipe<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">, z.ZodMiniTransform<... | undefined, number & z.$brand<...>>>>;
			epoch: z.ZodMiniOptional<z.ZodMiniType<ReferenceEpoch, string, z.core.$ZodTypeInternals<ReferenceEpoch, string>>>;
			jitter: z.ZodMiniOptional<z.ZodMiniPipe<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">, z.ZodMiniTransform<... | undefined, number & z.$brand<...>>>>;
			mode: z.ZodMiniString<string>;
			schema: z.ZodMiniOptional<z.ZodMiniString<string>>;
			track: z.ZodMiniOptional<z.ZodMiniString<string>>;
		}, z.core.$loose>>;
	};
	stats?: {
		track: string;
	};
	text?: {
		renditions: z.ZodMiniRecord<z.ZodMiniString<string>, z.ZodMiniObject<{
			broadcast: z.ZodMiniOptional<z.ZodMiniType<RelativeBroadcast, string, z.core.$ZodTypeInternals<RelativeBroadcast, string>>>;
			container: z.ZodMiniDefault<z.ZodMiniUnion<readonly [z.ZodMiniDiscriminatedUnion<..., ...>, z.ZodMiniPipe<..., ...>]>>;
			delay: z.ZodMiniOptional<z.ZodMiniPipe<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">, z.ZodMiniTransform<... | undefined, number & z.$brand<...>>>>;
			epoch: z.ZodMiniOptional<z.ZodMiniType<ReferenceEpoch, string, z.core.$ZodTypeInternals<ReferenceEpoch, string>>>;
			format: z.ZodMiniString<string>;
			jitter: z.ZodMiniOptional<z.ZodMiniPipe<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">, z.ZodMiniTransform<... | undefined, number & z.$brand<...>>>>;
			label: z.ZodMiniOptional<z.ZodMiniString<string>>;
			lang: z.ZodMiniOptional<z.ZodMiniString<string>>;
			role: z.ZodMiniDefault<z.ZodMiniString<string>>;
			track: z.ZodMiniOptional<z.ZodMiniString<string>>;
		}, z.core.$strip>>;
	};
	version?: number;
	video?: {
		display: {
			height: number & $brand<"u53">;
			width: number & $brand<"u53">;
		} | undefined;
		flip: undefined;
		renditions: {
			[k: string]: {
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
					range?: "full" | "limited";
					transfer?: number & $brand<"u8">;
				};
				container: {
					kind: "legacy";
				} | {
					init: string;
					kind: "cmaf";
				} | {
					kind: "loc";
				} | {
					kind: "unknown";
					raw: {
						[x: string]: unknown;
						kind: string;
					};
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
					kind: "rectilinear";
				} | {
					fov?: number;
					kind: "equirectangular";
				} | {
					kind: "unknown";
					raw: {
						[x: string]: unknown;
						kind: string;
					};
				};
				reorder?: number & $brand<"u8">;
				stereo?: string;
				track?: string;
				warmup?: number & $brand<"u53">;
			};
		};
		rotation: undefined;
	} | {
		display?: {
			height: number & $brand<"u53">;
			width: number & $brand<"u53">;
		};
		flip?: boolean;
		renditions: z.ZodMiniRecord<z.ZodMiniString<string>, z.ZodMiniObject<{
			bitrate: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>;
			broadcast: z.ZodMiniOptional<z.ZodMiniType<RelativeBroadcast, string, z.core.$ZodTypeInternals<RelativeBroadcast, string>>>;
			codec: z.ZodMiniString<string>;
			codedHeight: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>;
			codedWidth: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>;
			color: z.ZodMiniOptional<z.ZodMiniObject<{
				contentLight: z.ZodMiniOptional<z.ZodMiniObject<..., ...>>;
				mastering: z.ZodMiniOptional<z.ZodMiniObject<..., ...>>;
				matrix: z.ZodMiniOptional<z.core.$ZodBranded<..., ..., ...>>;
				primaries: z.ZodMiniOptional<z.core.$ZodBranded<..., ..., ...>>;
				range: z.ZodMiniOptional<z.ZodMiniEnum<...>>;
				transfer: z.ZodMiniOptional<z.core.$ZodBranded<..., ..., ...>>;
			}, z.core.$strip>>;
			container: z.ZodMiniDefault<z.ZodMiniUnion<readonly [z.ZodMiniDiscriminatedUnion<..., ...>, z.ZodMiniPipe<..., ...>]>>;
			delay: z.ZodMiniOptional<z.ZodMiniPipe<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">, z.ZodMiniTransform<... | undefined, number & z.$brand<...>>>>;
			description: z.ZodMiniOptional<z.ZodMiniString<string>>;
			displayAspectHeight: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>;
			displayAspectWidth: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>;
			enabled: z.ZodMiniOptional<z.ZodMiniBoolean<boolean>>;
			epoch: z.ZodMiniOptional<z.ZodMiniType<ReferenceEpoch, string, z.core.$ZodTypeInternals<ReferenceEpoch, string>>>;
			framerate: z.ZodMiniOptional<z.ZodMiniNumber<number>>;
			interlaced: z.ZodMiniOptional<z.ZodMiniBoolean<boolean>>;
			jitter: z.ZodMiniOptional<z.ZodMiniPipe<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">, z.ZodMiniTransform<... | undefined, number & z.$brand<...>>>>;
			label: z.ZodMiniOptional<z.ZodMiniString<string>>;
			optimizeForLatency: z.ZodMiniOptional<z.ZodMiniBoolean<boolean>>;
			preference: z.ZodMiniOptional<z.ZodMiniInt32>;
			projection: z.ZodMiniOptional<z.ZodMiniUnion<readonly [z.ZodMiniDiscriminatedUnion<..., ...>, z.ZodMiniPipe<..., ...>]>>;
			reorder: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u8", "out">>;
			stereo: z.ZodMiniOptional<z.ZodMiniString<string>>;
			track: z.ZodMiniOptional<z.ZodMiniString<string>>;
			warmup: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>;
		}, z.core.$strip>>;
		rotation?: number;
	};
}): {
	[key: string]: unknown;
	archive?: {
		replay?: RelativeBroadcast;
		replayEpoch?: ReferenceEpoch;
		store?: string;
		timelines: z.ZodMiniRecord<z.ZodMiniString<string>, z.ZodMiniObject<{
			duration: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>;
			track: z.ZodMiniString<string>;
		}, z.core.$strip>>;
		timescale: number & $brand<"u53">;
		version?: number & $brand<"u53">;
	};
	audio?: {
		renditions: {
			[x: string]: {
				bitrate?: number & $brand<"u53">;
				broadcast?: RelativeBroadcast;
				codec: string;
				container: {
					kind: "legacy";
				} | {
					init: string;
					kind: "cmaf";
				} | {
					kind: "loc";
				} | {
					kind: "unknown";
					raw: {
						[x: string]: unknown;
						kind: string;
					};
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
			};
		};
	} | {
		renditions: z.ZodMiniRecord<z.ZodMiniString<string>, z.ZodMiniObject<{
			bitrate: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>;
			broadcast: z.ZodMiniOptional<z.ZodMiniType<RelativeBroadcast, string, z.core.$ZodTypeInternals<RelativeBroadcast, string>>>;
			codec: z.ZodMiniString<string>;
			container: z.ZodMiniDefault<z.ZodMiniUnion<readonly [z.ZodMiniDiscriminatedUnion<..., ...>, z.ZodMiniPipe<..., ...>]>>;
			delay: z.ZodMiniOptional<z.ZodMiniPipe<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">, z.ZodMiniTransform<... | undefined, number & z.$brand<...>>>>;
			description: z.ZodMiniOptional<z.ZodMiniString<string>>;
			enabled: z.ZodMiniOptional<z.ZodMiniBoolean<boolean>>;
			epoch: z.ZodMiniOptional<z.ZodMiniType<ReferenceEpoch, string, z.core.$ZodTypeInternals<ReferenceEpoch, string>>>;
			jitter: z.ZodMiniOptional<z.ZodMiniPipe<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">, z.ZodMiniTransform<... | undefined, number & z.$brand<...>>>>;
			label: z.ZodMiniOptional<z.ZodMiniString<string>>;
			layout: z.ZodMiniOptional<z.ZodMiniString<string>>;
			numberOfChannels: z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">;
			sampleRate: z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">;
			track: z.ZodMiniOptional<z.ZodMiniString<string>>;
			warmup: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>;
		}, z.core.$strip>>;
	};
	binary?: {
		tracks: z.ZodMiniRecord<z.ZodMiniString<string>, z.ZodMiniObject<{
			bitrate: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>;
			broadcast: z.ZodMiniOptional<z.ZodMiniType<RelativeBroadcast, string, z.core.$ZodTypeInternals<RelativeBroadcast, string>>>;
			compression: z.ZodMiniOptional<z.ZodMiniString<string>>;
			delay: z.ZodMiniOptional<z.ZodMiniPipe<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">, z.ZodMiniTransform<... | undefined, number & z.$brand<...>>>>;
			epoch: z.ZodMiniOptional<z.ZodMiniType<ReferenceEpoch, string, z.core.$ZodTypeInternals<ReferenceEpoch, string>>>;
			jitter: z.ZodMiniOptional<z.ZodMiniPipe<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">, z.ZodMiniTransform<... | undefined, number & z.$brand<...>>>>;
			mime: z.ZodMiniOptional<z.ZodMiniString<string>>;
			mode: z.ZodMiniString<string>;
			track: z.ZodMiniOptional<z.ZodMiniString<string>>;
		}, z.core.$loose>>;
	};
	clock?: {
		timescale: number;
		wall: number & $brand<"u53">;
	};
	echo?: {
		path: RelativeBroadcast;
	};
	json?: {
		tracks: z.ZodMiniRecord<z.ZodMiniString<string>, z.ZodMiniObject<{
			bitrate: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>;
			broadcast: z.ZodMiniOptional<z.ZodMiniType<RelativeBroadcast, string, z.core.$ZodTypeInternals<RelativeBroadcast, string>>>;
			compression: z.ZodMiniOptional<z.ZodMiniString<string>>;
			delay: z.ZodMiniOptional<z.ZodMiniPipe<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">, z.ZodMiniTransform<... | undefined, number & z.$brand<...>>>>;
			epoch: z.ZodMiniOptional<z.ZodMiniType<ReferenceEpoch, string, z.core.$ZodTypeInternals<ReferenceEpoch, string>>>;
			jitter: z.ZodMiniOptional<z.ZodMiniPipe<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">, z.ZodMiniTransform<... | undefined, number & z.$brand<...>>>>;
			mode: z.ZodMiniString<string>;
			schema: z.ZodMiniOptional<z.ZodMiniString<string>>;
			track: z.ZodMiniOptional<z.ZodMiniString<string>>;
		}, z.core.$loose>>;
	};
	stats?: {
		track: string;
	};
	text?: {
		renditions: z.ZodMiniRecord<z.ZodMiniString<string>, z.ZodMiniObject<{
			broadcast: z.ZodMiniOptional<z.ZodMiniType<RelativeBroadcast, string, z.core.$ZodTypeInternals<RelativeBroadcast, string>>>;
			container: z.ZodMiniDefault<z.ZodMiniUnion<readonly [z.ZodMiniDiscriminatedUnion<..., ...>, z.ZodMiniPipe<..., ...>]>>;
			delay: z.ZodMiniOptional<z.ZodMiniPipe<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">, z.ZodMiniTransform<... | undefined, number & z.$brand<...>>>>;
			epoch: z.ZodMiniOptional<z.ZodMiniType<ReferenceEpoch, string, z.core.$ZodTypeInternals<ReferenceEpoch, string>>>;
			format: z.ZodMiniString<string>;
			jitter: z.ZodMiniOptional<z.ZodMiniPipe<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">, z.ZodMiniTransform<... | undefined, number & z.$brand<...>>>>;
			label: z.ZodMiniOptional<z.ZodMiniString<string>>;
			lang: z.ZodMiniOptional<z.ZodMiniString<string>>;
			role: z.ZodMiniDefault<z.ZodMiniString<string>>;
			track: z.ZodMiniOptional<z.ZodMiniString<string>>;
		}, z.core.$strip>>;
	};
	version?: number;
	video?: {
		display: {
			height: number & $brand<"u53">;
			width: number & $brand<"u53">;
		} | undefined;
		flip: undefined;
		renditions: {
			[k: string]: {
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
					range?: "full" | "limited";
					transfer?: number & $brand<"u8">;
				};
				container: {
					kind: "legacy";
				} | {
					init: string;
					kind: "cmaf";
				} | {
					kind: "loc";
				} | {
					kind: "unknown";
					raw: {
						[x: string]: unknown;
						kind: string;
					};
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
					kind: "rectilinear";
				} | {
					fov?: number;
					kind: "equirectangular";
				} | {
					kind: "unknown";
					raw: {
						[x: string]: unknown;
						kind: string;
					};
				};
				reorder?: number & $brand<"u8">;
				stereo?: string;
				track?: string;
				warmup?: number & $brand<"u53">;
			};
		};
		rotation: undefined;
	} | {
		display?: {
			height: number & $brand<"u53">;
			width: number & $brand<"u53">;
		};
		flip?: boolean;
		renditions: z.ZodMiniRecord<z.ZodMiniString<string>, z.ZodMiniObject<{
			bitrate: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>;
			broadcast: z.ZodMiniOptional<z.ZodMiniType<RelativeBroadcast, string, z.core.$ZodTypeInternals<RelativeBroadcast, string>>>;
			codec: z.ZodMiniString<string>;
			codedHeight: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>;
			codedWidth: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>;
			color: z.ZodMiniOptional<z.ZodMiniObject<{
				contentLight: z.ZodMiniOptional<z.ZodMiniObject<..., ...>>;
				mastering: z.ZodMiniOptional<z.ZodMiniObject<..., ...>>;
				matrix: z.ZodMiniOptional<z.core.$ZodBranded<..., ..., ...>>;
				primaries: z.ZodMiniOptional<z.core.$ZodBranded<..., ..., ...>>;
				range: z.ZodMiniOptional<z.ZodMiniEnum<...>>;
				transfer: z.ZodMiniOptional<z.core.$ZodBranded<..., ..., ...>>;
			}, z.core.$strip>>;
			container: z.ZodMiniDefault<z.ZodMiniUnion<readonly [z.ZodMiniDiscriminatedUnion<..., ...>, z.ZodMiniPipe<..., ...>]>>;
			delay: z.ZodMiniOptional<z.ZodMiniPipe<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">, z.ZodMiniTransform<... | undefined, number & z.$brand<...>>>>;
			description: z.ZodMiniOptional<z.ZodMiniString<string>>;
			displayAspectHeight: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>;
			displayAspectWidth: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>;
			enabled: z.ZodMiniOptional<z.ZodMiniBoolean<boolean>>;
			epoch: z.ZodMiniOptional<z.ZodMiniType<ReferenceEpoch, string, z.core.$ZodTypeInternals<ReferenceEpoch, string>>>;
			framerate: z.ZodMiniOptional<z.ZodMiniNumber<number>>;
			interlaced: z.ZodMiniOptional<z.ZodMiniBoolean<boolean>>;
			jitter: z.ZodMiniOptional<z.ZodMiniPipe<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">, z.ZodMiniTransform<... | undefined, number & z.$brand<...>>>>;
			label: z.ZodMiniOptional<z.ZodMiniString<string>>;
			optimizeForLatency: z.ZodMiniOptional<z.ZodMiniBoolean<boolean>>;
			preference: z.ZodMiniOptional<z.ZodMiniInt32>;
			projection: z.ZodMiniOptional<z.ZodMiniUnion<readonly [z.ZodMiniDiscriminatedUnion<..., ...>, z.ZodMiniPipe<..., ...>]>>;
			reorder: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u8", "out">>;
			stereo: z.ZodMiniOptional<z.ZodMiniString<string>>;
			track: z.ZodMiniOptional<z.ZodMiniString<string>>;
			warmup: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>;
		}, z.core.$strip>>;
		rotation?: number;
	};
};

export function checkResolvable(root: {
	[key: string]: unknown;
	archive?: {
		replay?: RelativeBroadcast;
		replayEpoch?: ReferenceEpoch;
		store?: string;
		timelines: z.ZodMiniRecord<z.ZodMiniString<string>, z.ZodMiniObject<{
			duration: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>;
			track: z.ZodMiniString<string>;
		}, z.core.$strip>>;
		timescale: number & $brand<"u53">;
		version?: number & $brand<"u53">;
	};
	audio?: {
		renditions: {
			[x: string]: {
				bitrate?: number & $brand<"u53">;
				broadcast?: RelativeBroadcast;
				codec: string;
				container: {
					kind: "legacy";
				} | {
					init: string;
					kind: "cmaf";
				} | {
					kind: "loc";
				} | {
					kind: "unknown";
					raw: {
						[x: string]: unknown;
						kind: string;
					};
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
			};
		};
	} | {
		renditions: z.ZodMiniRecord<z.ZodMiniString<string>, z.ZodMiniObject<{
			bitrate: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>;
			broadcast: z.ZodMiniOptional<z.ZodMiniType<RelativeBroadcast, string, z.core.$ZodTypeInternals<RelativeBroadcast, string>>>;
			codec: z.ZodMiniString<string>;
			container: z.ZodMiniDefault<z.ZodMiniUnion<readonly [z.ZodMiniDiscriminatedUnion<..., ...>, z.ZodMiniPipe<..., ...>]>>;
			delay: z.ZodMiniOptional<z.ZodMiniPipe<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">, z.ZodMiniTransform<... | undefined, number & z.$brand<...>>>>;
			description: z.ZodMiniOptional<z.ZodMiniString<string>>;
			enabled: z.ZodMiniOptional<z.ZodMiniBoolean<boolean>>;
			epoch: z.ZodMiniOptional<z.ZodMiniType<ReferenceEpoch, string, z.core.$ZodTypeInternals<ReferenceEpoch, string>>>;
			jitter: z.ZodMiniOptional<z.ZodMiniPipe<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">, z.ZodMiniTransform<... | undefined, number & z.$brand<...>>>>;
			label: z.ZodMiniOptional<z.ZodMiniString<string>>;
			layout: z.ZodMiniOptional<z.ZodMiniString<string>>;
			numberOfChannels: z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">;
			sampleRate: z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">;
			track: z.ZodMiniOptional<z.ZodMiniString<string>>;
			warmup: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>;
		}, z.core.$strip>>;
	};
	binary?: {
		tracks: z.ZodMiniRecord<z.ZodMiniString<string>, z.ZodMiniObject<{
			bitrate: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>;
			broadcast: z.ZodMiniOptional<z.ZodMiniType<RelativeBroadcast, string, z.core.$ZodTypeInternals<RelativeBroadcast, string>>>;
			compression: z.ZodMiniOptional<z.ZodMiniString<string>>;
			delay: z.ZodMiniOptional<z.ZodMiniPipe<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">, z.ZodMiniTransform<... | undefined, number & z.$brand<...>>>>;
			epoch: z.ZodMiniOptional<z.ZodMiniType<ReferenceEpoch, string, z.core.$ZodTypeInternals<ReferenceEpoch, string>>>;
			jitter: z.ZodMiniOptional<z.ZodMiniPipe<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">, z.ZodMiniTransform<... | undefined, number & z.$brand<...>>>>;
			mime: z.ZodMiniOptional<z.ZodMiniString<string>>;
			mode: z.ZodMiniString<string>;
			track: z.ZodMiniOptional<z.ZodMiniString<string>>;
		}, z.core.$loose>>;
	};
	clock?: {
		timescale: number;
		wall: number & $brand<"u53">;
	};
	echo?: {
		path: RelativeBroadcast;
	};
	json?: {
		tracks: z.ZodMiniRecord<z.ZodMiniString<string>, z.ZodMiniObject<{
			bitrate: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>;
			broadcast: z.ZodMiniOptional<z.ZodMiniType<RelativeBroadcast, string, z.core.$ZodTypeInternals<RelativeBroadcast, string>>>;
			compression: z.ZodMiniOptional<z.ZodMiniString<string>>;
			delay: z.ZodMiniOptional<z.ZodMiniPipe<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">, z.ZodMiniTransform<... | undefined, number & z.$brand<...>>>>;
			epoch: z.ZodMiniOptional<z.ZodMiniType<ReferenceEpoch, string, z.core.$ZodTypeInternals<ReferenceEpoch, string>>>;
			jitter: z.ZodMiniOptional<z.ZodMiniPipe<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">, z.ZodMiniTransform<... | undefined, number & z.$brand<...>>>>;
			mode: z.ZodMiniString<string>;
			schema: z.ZodMiniOptional<z.ZodMiniString<string>>;
			track: z.ZodMiniOptional<z.ZodMiniString<string>>;
		}, z.core.$loose>>;
	};
	stats?: {
		track: string;
	};
	text?: {
		renditions: z.ZodMiniRecord<z.ZodMiniString<string>, z.ZodMiniObject<{
			broadcast: z.ZodMiniOptional<z.ZodMiniType<RelativeBroadcast, string, z.core.$ZodTypeInternals<RelativeBroadcast, string>>>;
			container: z.ZodMiniDefault<z.ZodMiniUnion<readonly [z.ZodMiniDiscriminatedUnion<..., ...>, z.ZodMiniPipe<..., ...>]>>;
			delay: z.ZodMiniOptional<z.ZodMiniPipe<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">, z.ZodMiniTransform<... | undefined, number & z.$brand<...>>>>;
			epoch: z.ZodMiniOptional<z.ZodMiniType<ReferenceEpoch, string, z.core.$ZodTypeInternals<ReferenceEpoch, string>>>;
			format: z.ZodMiniString<string>;
			jitter: z.ZodMiniOptional<z.ZodMiniPipe<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">, z.ZodMiniTransform<... | undefined, number & z.$brand<...>>>>;
			label: z.ZodMiniOptional<z.ZodMiniString<string>>;
			lang: z.ZodMiniOptional<z.ZodMiniString<string>>;
			role: z.ZodMiniDefault<z.ZodMiniString<string>>;
			track: z.ZodMiniOptional<z.ZodMiniString<string>>;
		}, z.core.$strip>>;
	};
	version?: number;
	video?: {
		display: {
			height: number & $brand<"u53">;
			width: number & $brand<"u53">;
		} | undefined;
		flip: undefined;
		renditions: {
			[k: string]: {
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
					range?: "full" | "limited";
					transfer?: number & $brand<"u8">;
				};
				container: {
					kind: "legacy";
				} | {
					init: string;
					kind: "cmaf";
				} | {
					kind: "loc";
				} | {
					kind: "unknown";
					raw: {
						[x: string]: unknown;
						kind: string;
					};
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
					kind: "rectilinear";
				} | {
					fov?: number;
					kind: "equirectangular";
				} | {
					kind: "unknown";
					raw: {
						[x: string]: unknown;
						kind: string;
					};
				};
				reorder?: number & $brand<"u8">;
				stereo?: string;
				track?: string;
				warmup?: number & $brand<"u53">;
			};
		};
		rotation: undefined;
	} | {
		display?: {
			height: number & $brand<"u53">;
			width: number & $brand<"u53">;
		};
		flip?: boolean;
		renditions: z.ZodMiniRecord<z.ZodMiniString<string>, z.ZodMiniObject<{
			bitrate: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>;
			broadcast: z.ZodMiniOptional<z.ZodMiniType<RelativeBroadcast, string, z.core.$ZodTypeInternals<RelativeBroadcast, string>>>;
			codec: z.ZodMiniString<string>;
			codedHeight: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>;
			codedWidth: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>;
			color: z.ZodMiniOptional<z.ZodMiniObject<{
				contentLight: z.ZodMiniOptional<z.ZodMiniObject<..., ...>>;
				mastering: z.ZodMiniOptional<z.ZodMiniObject<..., ...>>;
				matrix: z.ZodMiniOptional<z.core.$ZodBranded<..., ..., ...>>;
				primaries: z.ZodMiniOptional<z.core.$ZodBranded<..., ..., ...>>;
				range: z.ZodMiniOptional<z.ZodMiniEnum<...>>;
				transfer: z.ZodMiniOptional<z.core.$ZodBranded<..., ..., ...>>;
			}, z.core.$strip>>;
			container: z.ZodMiniDefault<z.ZodMiniUnion<readonly [z.ZodMiniDiscriminatedUnion<..., ...>, z.ZodMiniPipe<..., ...>]>>;
			delay: z.ZodMiniOptional<z.ZodMiniPipe<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">, z.ZodMiniTransform<... | undefined, number & z.$brand<...>>>>;
			description: z.ZodMiniOptional<z.ZodMiniString<string>>;
			displayAspectHeight: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>;
			displayAspectWidth: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>;
			enabled: z.ZodMiniOptional<z.ZodMiniBoolean<boolean>>;
			epoch: z.ZodMiniOptional<z.ZodMiniType<ReferenceEpoch, string, z.core.$ZodTypeInternals<ReferenceEpoch, string>>>;
			framerate: z.ZodMiniOptional<z.ZodMiniNumber<number>>;
			interlaced: z.ZodMiniOptional<z.ZodMiniBoolean<boolean>>;
			jitter: z.ZodMiniOptional<z.ZodMiniPipe<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">, z.ZodMiniTransform<... | undefined, number & z.$brand<...>>>>;
			label: z.ZodMiniOptional<z.ZodMiniString<string>>;
			optimizeForLatency: z.ZodMiniOptional<z.ZodMiniBoolean<boolean>>;
			preference: z.ZodMiniOptional<z.ZodMiniInt32>;
			projection: z.ZodMiniOptional<z.ZodMiniUnion<readonly [z.ZodMiniDiscriminatedUnion<..., ...>, z.ZodMiniPipe<..., ...>]>>;
			reorder: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u8", "out">>;
			stereo: z.ZodMiniOptional<z.ZodMiniString<string>>;
			track: z.ZodMiniOptional<z.ZodMiniString<string>>;
			warmup: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>;
		}, z.core.$strip>>;
		rotation?: number;
	};
}, base: Valid): {
	[key: string]: unknown;
	archive?: {
		replay?: RelativeBroadcast;
		replayEpoch?: ReferenceEpoch;
		store?: string;
		timelines: z.ZodMiniRecord<z.ZodMiniString<string>, z.ZodMiniObject<{
			duration: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>;
			track: z.ZodMiniString<string>;
		}, z.core.$strip>>;
		timescale: number & $brand<"u53">;
		version?: number & $brand<"u53">;
	};
	audio?: {
		renditions: {
			[x: string]: {
				bitrate?: number & $brand<"u53">;
				broadcast?: RelativeBroadcast;
				codec: string;
				container: {
					kind: "legacy";
				} | {
					init: string;
					kind: "cmaf";
				} | {
					kind: "loc";
				} | {
					kind: "unknown";
					raw: {
						[x: string]: unknown;
						kind: string;
					};
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
			};
		};
	} | {
		renditions: z.ZodMiniRecord<z.ZodMiniString<string>, z.ZodMiniObject<{
			bitrate: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>;
			broadcast: z.ZodMiniOptional<z.ZodMiniType<RelativeBroadcast, string, z.core.$ZodTypeInternals<RelativeBroadcast, string>>>;
			codec: z.ZodMiniString<string>;
			container: z.ZodMiniDefault<z.ZodMiniUnion<readonly [z.ZodMiniDiscriminatedUnion<..., ...>, z.ZodMiniPipe<..., ...>]>>;
			delay: z.ZodMiniOptional<z.ZodMiniPipe<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">, z.ZodMiniTransform<... | undefined, number & z.$brand<...>>>>;
			description: z.ZodMiniOptional<z.ZodMiniString<string>>;
			enabled: z.ZodMiniOptional<z.ZodMiniBoolean<boolean>>;
			epoch: z.ZodMiniOptional<z.ZodMiniType<ReferenceEpoch, string, z.core.$ZodTypeInternals<ReferenceEpoch, string>>>;
			jitter: z.ZodMiniOptional<z.ZodMiniPipe<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">, z.ZodMiniTransform<... | undefined, number & z.$brand<...>>>>;
			label: z.ZodMiniOptional<z.ZodMiniString<string>>;
			layout: z.ZodMiniOptional<z.ZodMiniString<string>>;
			numberOfChannels: z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">;
			sampleRate: z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">;
			track: z.ZodMiniOptional<z.ZodMiniString<string>>;
			warmup: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>;
		}, z.core.$strip>>;
	};
	binary?: {
		tracks: z.ZodMiniRecord<z.ZodMiniString<string>, z.ZodMiniObject<{
			bitrate: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>;
			broadcast: z.ZodMiniOptional<z.ZodMiniType<RelativeBroadcast, string, z.core.$ZodTypeInternals<RelativeBroadcast, string>>>;
			compression: z.ZodMiniOptional<z.ZodMiniString<string>>;
			delay: z.ZodMiniOptional<z.ZodMiniPipe<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">, z.ZodMiniTransform<... | undefined, number & z.$brand<...>>>>;
			epoch: z.ZodMiniOptional<z.ZodMiniType<ReferenceEpoch, string, z.core.$ZodTypeInternals<ReferenceEpoch, string>>>;
			jitter: z.ZodMiniOptional<z.ZodMiniPipe<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">, z.ZodMiniTransform<... | undefined, number & z.$brand<...>>>>;
			mime: z.ZodMiniOptional<z.ZodMiniString<string>>;
			mode: z.ZodMiniString<string>;
			track: z.ZodMiniOptional<z.ZodMiniString<string>>;
		}, z.core.$loose>>;
	};
	clock?: {
		timescale: number;
		wall: number & $brand<"u53">;
	};
	echo?: {
		path: RelativeBroadcast;
	};
	json?: {
		tracks: z.ZodMiniRecord<z.ZodMiniString<string>, z.ZodMiniObject<{
			bitrate: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>;
			broadcast: z.ZodMiniOptional<z.ZodMiniType<RelativeBroadcast, string, z.core.$ZodTypeInternals<RelativeBroadcast, string>>>;
			compression: z.ZodMiniOptional<z.ZodMiniString<string>>;
			delay: z.ZodMiniOptional<z.ZodMiniPipe<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">, z.ZodMiniTransform<... | undefined, number & z.$brand<...>>>>;
			epoch: z.ZodMiniOptional<z.ZodMiniType<ReferenceEpoch, string, z.core.$ZodTypeInternals<ReferenceEpoch, string>>>;
			jitter: z.ZodMiniOptional<z.ZodMiniPipe<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">, z.ZodMiniTransform<... | undefined, number & z.$brand<...>>>>;
			mode: z.ZodMiniString<string>;
			schema: z.ZodMiniOptional<z.ZodMiniString<string>>;
			track: z.ZodMiniOptional<z.ZodMiniString<string>>;
		}, z.core.$loose>>;
	};
	stats?: {
		track: string;
	};
	text?: {
		renditions: z.ZodMiniRecord<z.ZodMiniString<string>, z.ZodMiniObject<{
			broadcast: z.ZodMiniOptional<z.ZodMiniType<RelativeBroadcast, string, z.core.$ZodTypeInternals<RelativeBroadcast, string>>>;
			container: z.ZodMiniDefault<z.ZodMiniUnion<readonly [z.ZodMiniDiscriminatedUnion<..., ...>, z.ZodMiniPipe<..., ...>]>>;
			delay: z.ZodMiniOptional<z.ZodMiniPipe<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">, z.ZodMiniTransform<... | undefined, number & z.$brand<...>>>>;
			epoch: z.ZodMiniOptional<z.ZodMiniType<ReferenceEpoch, string, z.core.$ZodTypeInternals<ReferenceEpoch, string>>>;
			format: z.ZodMiniString<string>;
			jitter: z.ZodMiniOptional<z.ZodMiniPipe<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">, z.ZodMiniTransform<... | undefined, number & z.$brand<...>>>>;
			label: z.ZodMiniOptional<z.ZodMiniString<string>>;
			lang: z.ZodMiniOptional<z.ZodMiniString<string>>;
			role: z.ZodMiniDefault<z.ZodMiniString<string>>;
			track: z.ZodMiniOptional<z.ZodMiniString<string>>;
		}, z.core.$strip>>;
	};
	version?: number;
	video?: {
		display: {
			height: number & $brand<"u53">;
			width: number & $brand<"u53">;
		} | undefined;
		flip: undefined;
		renditions: {
			[k: string]: {
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
					range?: "full" | "limited";
					transfer?: number & $brand<"u8">;
				};
				container: {
					kind: "legacy";
				} | {
					init: string;
					kind: "cmaf";
				} | {
					kind: "loc";
				} | {
					kind: "unknown";
					raw: {
						[x: string]: unknown;
						kind: string;
					};
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
					kind: "rectilinear";
				} | {
					fov?: number;
					kind: "equirectangular";
				} | {
					kind: "unknown";
					raw: {
						[x: string]: unknown;
						kind: string;
					};
				};
				reorder?: number & $brand<"u8">;
				stereo?: string;
				track?: string;
				warmup?: number & $brand<"u53">;
			};
		};
		rotation: undefined;
	} | {
		display?: {
			height: number & $brand<"u53">;
			width: number & $brand<"u53">;
		};
		flip?: boolean;
		renditions: z.ZodMiniRecord<z.ZodMiniString<string>, z.ZodMiniObject<{
			bitrate: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>;
			broadcast: z.ZodMiniOptional<z.ZodMiniType<RelativeBroadcast, string, z.core.$ZodTypeInternals<RelativeBroadcast, string>>>;
			codec: z.ZodMiniString<string>;
			codedHeight: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>;
			codedWidth: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>;
			color: z.ZodMiniOptional<z.ZodMiniObject<{
				contentLight: z.ZodMiniOptional<z.ZodMiniObject<..., ...>>;
				mastering: z.ZodMiniOptional<z.ZodMiniObject<..., ...>>;
				matrix: z.ZodMiniOptional<z.core.$ZodBranded<..., ..., ...>>;
				primaries: z.ZodMiniOptional<z.core.$ZodBranded<..., ..., ...>>;
				range: z.ZodMiniOptional<z.ZodMiniEnum<...>>;
				transfer: z.ZodMiniOptional<z.core.$ZodBranded<..., ..., ...>>;
			}, z.core.$strip>>;
			container: z.ZodMiniDefault<z.ZodMiniUnion<readonly [z.ZodMiniDiscriminatedUnion<..., ...>, z.ZodMiniPipe<..., ...>]>>;
			delay: z.ZodMiniOptional<z.ZodMiniPipe<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">, z.ZodMiniTransform<... | undefined, number & z.$brand<...>>>>;
			description: z.ZodMiniOptional<z.ZodMiniString<string>>;
			displayAspectHeight: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>;
			displayAspectWidth: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>;
			enabled: z.ZodMiniOptional<z.ZodMiniBoolean<boolean>>;
			epoch: z.ZodMiniOptional<z.ZodMiniType<ReferenceEpoch, string, z.core.$ZodTypeInternals<ReferenceEpoch, string>>>;
			framerate: z.ZodMiniOptional<z.ZodMiniNumber<number>>;
			interlaced: z.ZodMiniOptional<z.ZodMiniBoolean<boolean>>;
			jitter: z.ZodMiniOptional<z.ZodMiniPipe<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">, z.ZodMiniTransform<... | undefined, number & z.$brand<...>>>>;
			label: z.ZodMiniOptional<z.ZodMiniString<string>>;
			optimizeForLatency: z.ZodMiniOptional<z.ZodMiniBoolean<boolean>>;
			preference: z.ZodMiniOptional<z.ZodMiniInt32>;
			projection: z.ZodMiniOptional<z.ZodMiniUnion<readonly [z.ZodMiniDiscriminatedUnion<..., ...>, z.ZodMiniPipe<..., ...>]>>;
			reorder: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u8", "out">>;
			stereo: z.ZodMiniOptional<z.ZodMiniString<string>>;
			track: z.ZodMiniOptional<z.ZodMiniString<string>>;
			warmup: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>;
		}, z.core.$strip>>;
		rotation?: number;
	};
};

export function compressionSupported(compression: string): compression is "deflate";

export function containerSupported(container: {
	kind: "unknown";
	raw: {
		[x: string]: unknown;
		kind: string;
	};
} | {
	kind: "legacy";
} | {
	init: string;
	kind: "cmaf";
} | {
	kind: "loc";
}): boolean;

export function detectFormat(name: string): "hang" | "msf" | undefined;

export function encode(root: {
	[key: string]: unknown;
	archive?: {
		replay?: RelativeBroadcast;
		replayEpoch?: ReferenceEpoch;
		store?: string;
		timelines: z.ZodMiniRecord<z.ZodMiniString<string>, z.ZodMiniObject<{
			duration: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>;
			track: z.ZodMiniString<string>;
		}, z.core.$strip>>;
		timescale: number & $brand<"u53">;
		version?: number & $brand<"u53">;
	};
	audio?: {
		renditions: {
			[x: string]: {
				bitrate?: number & $brand<"u53">;
				broadcast?: RelativeBroadcast;
				codec: string;
				container: {
					kind: "legacy";
				} | {
					init: string;
					kind: "cmaf";
				} | {
					kind: "loc";
				} | {
					kind: "unknown";
					raw: {
						[x: string]: unknown;
						kind: string;
					};
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
			};
		};
	} | {
		renditions: z.ZodMiniRecord<z.ZodMiniString<string>, z.ZodMiniObject<{
			bitrate: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>;
			broadcast: z.ZodMiniOptional<z.ZodMiniType<RelativeBroadcast, string, z.core.$ZodTypeInternals<RelativeBroadcast, string>>>;
			codec: z.ZodMiniString<string>;
			container: z.ZodMiniDefault<z.ZodMiniUnion<readonly [z.ZodMiniDiscriminatedUnion<..., ...>, z.ZodMiniPipe<..., ...>]>>;
			delay: z.ZodMiniOptional<z.ZodMiniPipe<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">, z.ZodMiniTransform<... | undefined, number & z.$brand<...>>>>;
			description: z.ZodMiniOptional<z.ZodMiniString<string>>;
			enabled: z.ZodMiniOptional<z.ZodMiniBoolean<boolean>>;
			epoch: z.ZodMiniOptional<z.ZodMiniType<ReferenceEpoch, string, z.core.$ZodTypeInternals<ReferenceEpoch, string>>>;
			jitter: z.ZodMiniOptional<z.ZodMiniPipe<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">, z.ZodMiniTransform<... | undefined, number & z.$brand<...>>>>;
			label: z.ZodMiniOptional<z.ZodMiniString<string>>;
			layout: z.ZodMiniOptional<z.ZodMiniString<string>>;
			numberOfChannels: z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">;
			sampleRate: z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">;
			track: z.ZodMiniOptional<z.ZodMiniString<string>>;
			warmup: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>;
		}, z.core.$strip>>;
	};
	binary?: {
		tracks: z.ZodMiniRecord<z.ZodMiniString<string>, z.ZodMiniObject<{
			bitrate: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>;
			broadcast: z.ZodMiniOptional<z.ZodMiniType<RelativeBroadcast, string, z.core.$ZodTypeInternals<RelativeBroadcast, string>>>;
			compression: z.ZodMiniOptional<z.ZodMiniString<string>>;
			delay: z.ZodMiniOptional<z.ZodMiniPipe<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">, z.ZodMiniTransform<... | undefined, number & z.$brand<...>>>>;
			epoch: z.ZodMiniOptional<z.ZodMiniType<ReferenceEpoch, string, z.core.$ZodTypeInternals<ReferenceEpoch, string>>>;
			jitter: z.ZodMiniOptional<z.ZodMiniPipe<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">, z.ZodMiniTransform<... | undefined, number & z.$brand<...>>>>;
			mime: z.ZodMiniOptional<z.ZodMiniString<string>>;
			mode: z.ZodMiniString<string>;
			track: z.ZodMiniOptional<z.ZodMiniString<string>>;
		}, z.core.$loose>>;
	};
	clock?: {
		timescale: number;
		wall: number & $brand<"u53">;
	};
	echo?: {
		path: RelativeBroadcast;
	};
	json?: {
		tracks: z.ZodMiniRecord<z.ZodMiniString<string>, z.ZodMiniObject<{
			bitrate: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>;
			broadcast: z.ZodMiniOptional<z.ZodMiniType<RelativeBroadcast, string, z.core.$ZodTypeInternals<RelativeBroadcast, string>>>;
			compression: z.ZodMiniOptional<z.ZodMiniString<string>>;
			delay: z.ZodMiniOptional<z.ZodMiniPipe<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">, z.ZodMiniTransform<... | undefined, number & z.$brand<...>>>>;
			epoch: z.ZodMiniOptional<z.ZodMiniType<ReferenceEpoch, string, z.core.$ZodTypeInternals<ReferenceEpoch, string>>>;
			jitter: z.ZodMiniOptional<z.ZodMiniPipe<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">, z.ZodMiniTransform<... | undefined, number & z.$brand<...>>>>;
			mode: z.ZodMiniString<string>;
			schema: z.ZodMiniOptional<z.ZodMiniString<string>>;
			track: z.ZodMiniOptional<z.ZodMiniString<string>>;
		}, z.core.$loose>>;
	};
	stats?: {
		track: string;
	};
	text?: {
		renditions: z.ZodMiniRecord<z.ZodMiniString<string>, z.ZodMiniObject<{
			broadcast: z.ZodMiniOptional<z.ZodMiniType<RelativeBroadcast, string, z.core.$ZodTypeInternals<RelativeBroadcast, string>>>;
			container: z.ZodMiniDefault<z.ZodMiniUnion<readonly [z.ZodMiniDiscriminatedUnion<..., ...>, z.ZodMiniPipe<..., ...>]>>;
			delay: z.ZodMiniOptional<z.ZodMiniPipe<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">, z.ZodMiniTransform<... | undefined, number & z.$brand<...>>>>;
			epoch: z.ZodMiniOptional<z.ZodMiniType<ReferenceEpoch, string, z.core.$ZodTypeInternals<ReferenceEpoch, string>>>;
			format: z.ZodMiniString<string>;
			jitter: z.ZodMiniOptional<z.ZodMiniPipe<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">, z.ZodMiniTransform<... | undefined, number & z.$brand<...>>>>;
			label: z.ZodMiniOptional<z.ZodMiniString<string>>;
			lang: z.ZodMiniOptional<z.ZodMiniString<string>>;
			role: z.ZodMiniDefault<z.ZodMiniString<string>>;
			track: z.ZodMiniOptional<z.ZodMiniString<string>>;
		}, z.core.$strip>>;
	};
	version?: number;
	video?: {
		display: {
			height: number & $brand<"u53">;
			width: number & $brand<"u53">;
		} | undefined;
		flip: undefined;
		renditions: {
			[k: string]: {
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
					range?: "full" | "limited";
					transfer?: number & $brand<"u8">;
				};
				container: {
					kind: "legacy";
				} | {
					init: string;
					kind: "cmaf";
				} | {
					kind: "loc";
				} | {
					kind: "unknown";
					raw: {
						[x: string]: unknown;
						kind: string;
					};
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
					kind: "rectilinear";
				} | {
					fov?: number;
					kind: "equirectangular";
				} | {
					kind: "unknown";
					raw: {
						[x: string]: unknown;
						kind: string;
					};
				};
				reorder?: number & $brand<"u8">;
				stereo?: string;
				track?: string;
				warmup?: number & $brand<"u53">;
			};
		};
		rotation: undefined;
	} | {
		display?: {
			height: number & $brand<"u53">;
			width: number & $brand<"u53">;
		};
		flip?: boolean;
		renditions: z.ZodMiniRecord<z.ZodMiniString<string>, z.ZodMiniObject<{
			bitrate: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>;
			broadcast: z.ZodMiniOptional<z.ZodMiniType<RelativeBroadcast, string, z.core.$ZodTypeInternals<RelativeBroadcast, string>>>;
			codec: z.ZodMiniString<string>;
			codedHeight: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>;
			codedWidth: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>;
			color: z.ZodMiniOptional<z.ZodMiniObject<{
				contentLight: z.ZodMiniOptional<z.ZodMiniObject<..., ...>>;
				mastering: z.ZodMiniOptional<z.ZodMiniObject<..., ...>>;
				matrix: z.ZodMiniOptional<z.core.$ZodBranded<..., ..., ...>>;
				primaries: z.ZodMiniOptional<z.core.$ZodBranded<..., ..., ...>>;
				range: z.ZodMiniOptional<z.ZodMiniEnum<...>>;
				transfer: z.ZodMiniOptional<z.core.$ZodBranded<..., ..., ...>>;
			}, z.core.$strip>>;
			container: z.ZodMiniDefault<z.ZodMiniUnion<readonly [z.ZodMiniDiscriminatedUnion<..., ...>, z.ZodMiniPipe<..., ...>]>>;
			delay: z.ZodMiniOptional<z.ZodMiniPipe<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">, z.ZodMiniTransform<... | undefined, number & z.$brand<...>>>>;
			description: z.ZodMiniOptional<z.ZodMiniString<string>>;
			displayAspectHeight: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>;
			displayAspectWidth: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>;
			enabled: z.ZodMiniOptional<z.ZodMiniBoolean<boolean>>;
			epoch: z.ZodMiniOptional<z.ZodMiniType<ReferenceEpoch, string, z.core.$ZodTypeInternals<ReferenceEpoch, string>>>;
			framerate: z.ZodMiniOptional<z.ZodMiniNumber<number>>;
			interlaced: z.ZodMiniOptional<z.ZodMiniBoolean<boolean>>;
			jitter: z.ZodMiniOptional<z.ZodMiniPipe<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">, z.ZodMiniTransform<... | undefined, number & z.$brand<...>>>>;
			label: z.ZodMiniOptional<z.ZodMiniString<string>>;
			optimizeForLatency: z.ZodMiniOptional<z.ZodMiniBoolean<boolean>>;
			preference: z.ZodMiniOptional<z.ZodMiniInt32>;
			projection: z.ZodMiniOptional<z.ZodMiniUnion<readonly [z.ZodMiniDiscriminatedUnion<..., ...>, z.ZodMiniPipe<..., ...>]>>;
			reorder: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u8", "out">>;
			stereo: z.ZodMiniOptional<z.ZodMiniString<string>>;
			track: z.ZodMiniOptional<z.ZodMiniString<string>>;
			warmup: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>;
		}, z.core.$strip>>;
		rotation?: number;
	};
}): {
	[key: string]: unknown;
	archive?: {
		replay?: RelativeBroadcast;
		replayEpoch?: ReferenceEpoch;
		store?: string;
		timelines: z.ZodMiniRecord<z.ZodMiniString<string>, z.ZodMiniObject<{
			duration: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>;
			track: z.ZodMiniString<string>;
		}, z.core.$strip>>;
		timescale: number & $brand<"u53">;
		version?: number & $brand<"u53">;
	};
	audio?: {
		renditions: {
			[x: string]: {
				bitrate?: number & $brand<"u53">;
				broadcast?: RelativeBroadcast;
				codec: string;
				container: {
					kind: "legacy";
				} | {
					init: string;
					kind: "cmaf";
				} | {
					kind: "loc";
				} | {
					kind: "unknown";
					raw: {
						[x: string]: unknown;
						kind: string;
					};
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
			};
		};
	} | {
		renditions: z.ZodMiniRecord<z.ZodMiniString<string>, z.ZodMiniObject<{
			bitrate: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>;
			broadcast: z.ZodMiniOptional<z.ZodMiniType<RelativeBroadcast, string, z.core.$ZodTypeInternals<RelativeBroadcast, string>>>;
			codec: z.ZodMiniString<string>;
			container: z.ZodMiniDefault<z.ZodMiniUnion<readonly [z.ZodMiniDiscriminatedUnion<..., ...>, z.ZodMiniPipe<..., ...>]>>;
			delay: z.ZodMiniOptional<z.ZodMiniPipe<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">, z.ZodMiniTransform<... | undefined, number & z.$brand<...>>>>;
			description: z.ZodMiniOptional<z.ZodMiniString<string>>;
			enabled: z.ZodMiniOptional<z.ZodMiniBoolean<boolean>>;
			epoch: z.ZodMiniOptional<z.ZodMiniType<ReferenceEpoch, string, z.core.$ZodTypeInternals<ReferenceEpoch, string>>>;
			jitter: z.ZodMiniOptional<z.ZodMiniPipe<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">, z.ZodMiniTransform<... | undefined, number & z.$brand<...>>>>;
			label: z.ZodMiniOptional<z.ZodMiniString<string>>;
			layout: z.ZodMiniOptional<z.ZodMiniString<string>>;
			numberOfChannels: z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">;
			sampleRate: z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">;
			track: z.ZodMiniOptional<z.ZodMiniString<string>>;
			warmup: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>;
		}, z.core.$strip>>;
	};
	binary?: {
		tracks: z.ZodMiniRecord<z.ZodMiniString<string>, z.ZodMiniObject<{
			bitrate: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>;
			broadcast: z.ZodMiniOptional<z.ZodMiniType<RelativeBroadcast, string, z.core.$ZodTypeInternals<RelativeBroadcast, string>>>;
			compression: z.ZodMiniOptional<z.ZodMiniString<string>>;
			delay: z.ZodMiniOptional<z.ZodMiniPipe<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">, z.ZodMiniTransform<... | undefined, number & z.$brand<...>>>>;
			epoch: z.ZodMiniOptional<z.ZodMiniType<ReferenceEpoch, string, z.core.$ZodTypeInternals<ReferenceEpoch, string>>>;
			jitter: z.ZodMiniOptional<z.ZodMiniPipe<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">, z.ZodMiniTransform<... | undefined, number & z.$brand<...>>>>;
			mime: z.ZodMiniOptional<z.ZodMiniString<string>>;
			mode: z.ZodMiniString<string>;
			track: z.ZodMiniOptional<z.ZodMiniString<string>>;
		}, z.core.$loose>>;
	};
	clock?: {
		timescale: number;
		wall: number & $brand<"u53">;
	};
	echo?: {
		path: RelativeBroadcast;
	};
	json?: {
		tracks: z.ZodMiniRecord<z.ZodMiniString<string>, z.ZodMiniObject<{
			bitrate: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>;
			broadcast: z.ZodMiniOptional<z.ZodMiniType<RelativeBroadcast, string, z.core.$ZodTypeInternals<RelativeBroadcast, string>>>;
			compression: z.ZodMiniOptional<z.ZodMiniString<string>>;
			delay: z.ZodMiniOptional<z.ZodMiniPipe<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">, z.ZodMiniTransform<... | undefined, number & z.$brand<...>>>>;
			epoch: z.ZodMiniOptional<z.ZodMiniType<ReferenceEpoch, string, z.core.$ZodTypeInternals<ReferenceEpoch, string>>>;
			jitter: z.ZodMiniOptional<z.ZodMiniPipe<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">, z.ZodMiniTransform<... | undefined, number & z.$brand<...>>>>;
			mode: z.ZodMiniString<string>;
			schema: z.ZodMiniOptional<z.ZodMiniString<string>>;
			track: z.ZodMiniOptional<z.ZodMiniString<string>>;
		}, z.core.$loose>>;
	};
	stats?: {
		track: string;
	};
	text?: {
		renditions: z.ZodMiniRecord<z.ZodMiniString<string>, z.ZodMiniObject<{
			broadcast: z.ZodMiniOptional<z.ZodMiniType<RelativeBroadcast, string, z.core.$ZodTypeInternals<RelativeBroadcast, string>>>;
			container: z.ZodMiniDefault<z.ZodMiniUnion<readonly [z.ZodMiniDiscriminatedUnion<..., ...>, z.ZodMiniPipe<..., ...>]>>;
			delay: z.ZodMiniOptional<z.ZodMiniPipe<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">, z.ZodMiniTransform<... | undefined, number & z.$brand<...>>>>;
			epoch: z.ZodMiniOptional<z.ZodMiniType<ReferenceEpoch, string, z.core.$ZodTypeInternals<ReferenceEpoch, string>>>;
			format: z.ZodMiniString<string>;
			jitter: z.ZodMiniOptional<z.ZodMiniPipe<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">, z.ZodMiniTransform<... | undefined, number & z.$brand<...>>>>;
			label: z.ZodMiniOptional<z.ZodMiniString<string>>;
			lang: z.ZodMiniOptional<z.ZodMiniString<string>>;
			role: z.ZodMiniDefault<z.ZodMiniString<string>>;
			track: z.ZodMiniOptional<z.ZodMiniString<string>>;
		}, z.core.$strip>>;
	};
	version?: number;
	video?: {
		display: {
			height: number & $brand<"u53">;
			width: number & $brand<"u53">;
		} | undefined;
		flip: undefined;
		renditions: {
			[k: string]: {
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
					range?: "full" | "limited";
					transfer?: number & $brand<"u8">;
				};
				container: {
					kind: "legacy";
				} | {
					init: string;
					kind: "cmaf";
				} | {
					kind: "loc";
				} | {
					kind: "unknown";
					raw: {
						[x: string]: unknown;
						kind: string;
					};
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
					kind: "rectilinear";
				} | {
					fov?: number;
					kind: "equirectangular";
				} | {
					kind: "unknown";
					raw: {
						[x: string]: unknown;
						kind: string;
					};
				};
				reorder?: number & $brand<"u8">;
				stereo?: string;
				track?: string;
				warmup?: number & $brand<"u53">;
			};
		};
		rotation: undefined;
	} | {
		display?: {
			height: number & $brand<"u53">;
			width: number & $brand<"u53">;
		};
		flip?: boolean;
		renditions: z.ZodMiniRecord<z.ZodMiniString<string>, z.ZodMiniObject<{
			bitrate: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>;
			broadcast: z.ZodMiniOptional<z.ZodMiniType<RelativeBroadcast, string, z.core.$ZodTypeInternals<RelativeBroadcast, string>>>;
			codec: z.ZodMiniString<string>;
			codedHeight: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>;
			codedWidth: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>;
			color: z.ZodMiniOptional<z.ZodMiniObject<{
				contentLight: z.ZodMiniOptional<z.ZodMiniObject<..., ...>>;
				mastering: z.ZodMiniOptional<z.ZodMiniObject<..., ...>>;
				matrix: z.ZodMiniOptional<z.core.$ZodBranded<..., ..., ...>>;
				primaries: z.ZodMiniOptional<z.core.$ZodBranded<..., ..., ...>>;
				range: z.ZodMiniOptional<z.ZodMiniEnum<...>>;
				transfer: z.ZodMiniOptional<z.core.$ZodBranded<..., ..., ...>>;
			}, z.core.$strip>>;
			container: z.ZodMiniDefault<z.ZodMiniUnion<readonly [z.ZodMiniDiscriminatedUnion<..., ...>, z.ZodMiniPipe<..., ...>]>>;
			delay: z.ZodMiniOptional<z.ZodMiniPipe<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">, z.ZodMiniTransform<... | undefined, number & z.$brand<...>>>>;
			description: z.ZodMiniOptional<z.ZodMiniString<string>>;
			displayAspectHeight: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>;
			displayAspectWidth: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>;
			enabled: z.ZodMiniOptional<z.ZodMiniBoolean<boolean>>;
			epoch: z.ZodMiniOptional<z.ZodMiniType<ReferenceEpoch, string, z.core.$ZodTypeInternals<ReferenceEpoch, string>>>;
			framerate: z.ZodMiniOptional<z.ZodMiniNumber<number>>;
			interlaced: z.ZodMiniOptional<z.ZodMiniBoolean<boolean>>;
			jitter: z.ZodMiniOptional<z.ZodMiniPipe<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">, z.ZodMiniTransform<... | undefined, number & z.$brand<...>>>>;
			label: z.ZodMiniOptional<z.ZodMiniString<string>>;
			optimizeForLatency: z.ZodMiniOptional<z.ZodMiniBoolean<boolean>>;
			preference: z.ZodMiniOptional<z.ZodMiniInt32>;
			projection: z.ZodMiniOptional<z.ZodMiniUnion<readonly [z.ZodMiniDiscriminatedUnion<..., ...>, z.ZodMiniPipe<..., ...>]>>;
			reorder: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u8", "out">>;
			stereo: z.ZodMiniOptional<z.ZodMiniString<string>>;
			track: z.ZodMiniOptional<z.ZodMiniString<string>>;
			warmup: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>;
		}, z.core.$strip>>;
		rotation?: number;
	};
};

export const hexSchema: z.ZodMiniString<string>;

export function modeSupported(mode: string): mode is KnownMode;

export const nonzeroU53Schema: z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">;

export function track(id: string, config: {
	track?: string;
}): string;

export function u53(value: number): number & $brand<"u53">;

export const u53Schema: z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">;

export function u8(value: number): number & $brand<"u8">;

export const u8Schema: z.core.$ZodBranded<z.ZodMiniNumber<number>, "u8", "out">;

export function wallClockTime(clock: {
	timescale: number;
	wall: number & $brand<"u53">;
}, pts: number, ptsTimescale: number): Date;

export function watch(broadcast: Consumer): AsyncIterable<{
	[key: string]: unknown;
	archive?: {
		replay?: RelativeBroadcast;
		replayEpoch?: ReferenceEpoch;
		store?: string;
		timelines: z.ZodMiniRecord<z.ZodMiniString<string>, z.ZodMiniObject<{
			duration: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>;
			track: z.ZodMiniString<string>;
		}, z.core.$strip>>;
		timescale: number & $brand<"u53">;
		version?: number & $brand<"u53">;
	};
	audio?: {
		renditions: {
			[x: string]: {
				bitrate?: number & $brand<"u53">;
				broadcast?: RelativeBroadcast;
				codec: string;
				container: {
					kind: "legacy";
				} | {
					init: string;
					kind: "cmaf";
				} | {
					kind: "loc";
				} | {
					kind: "unknown";
					raw: {
						[x: string]: unknown;
						kind: string;
					};
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
			};
		};
	} | {
		renditions: z.ZodMiniRecord<z.ZodMiniString<string>, z.ZodMiniObject<{
			bitrate: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>;
			broadcast: z.ZodMiniOptional<z.ZodMiniType<RelativeBroadcast, string, z.core.$ZodTypeInternals<RelativeBroadcast, string>>>;
			codec: z.ZodMiniString<string>;
			container: z.ZodMiniDefault<z.ZodMiniUnion<readonly [..., ...]>>;
			delay: z.ZodMiniOptional<z.ZodMiniPipe<z.core.$ZodBranded<z.ZodMiniNumber<...>, "u53", "out">, z.ZodMiniTransform<... | ..., ... & ...>>>;
			description: z.ZodMiniOptional<z.ZodMiniString<string>>;
			enabled: z.ZodMiniOptional<z.ZodMiniBoolean<boolean>>;
			epoch: z.ZodMiniOptional<z.ZodMiniType<ReferenceEpoch, string, z.core.$ZodTypeInternals<ReferenceEpoch, string>>>;
			jitter: z.ZodMiniOptional<z.ZodMiniPipe<z.core.$ZodBranded<z.ZodMiniNumber<...>, "u53", "out">, z.ZodMiniTransform<... | ..., ... & ...>>>;
			label: z.ZodMiniOptional<z.ZodMiniString<string>>;
			layout: z.ZodMiniOptional<z.ZodMiniString<string>>;
			numberOfChannels: z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">;
			sampleRate: z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">;
			track: z.ZodMiniOptional<z.ZodMiniString<string>>;
			warmup: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>;
		}, z.core.$strip>>;
	};
	binary?: {
		tracks: z.ZodMiniRecord<z.ZodMiniString<string>, z.ZodMiniObject<{
			bitrate: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>;
			broadcast: z.ZodMiniOptional<z.ZodMiniType<RelativeBroadcast, string, z.core.$ZodTypeInternals<RelativeBroadcast, string>>>;
			compression: z.ZodMiniOptional<z.ZodMiniString<string>>;
			delay: z.ZodMiniOptional<z.ZodMiniPipe<z.core.$ZodBranded<z.ZodMiniNumber<...>, "u53", "out">, z.ZodMiniTransform<... | ..., ... & ...>>>;
			epoch: z.ZodMiniOptional<z.ZodMiniType<ReferenceEpoch, string, z.core.$ZodTypeInternals<ReferenceEpoch, string>>>;
			jitter: z.ZodMiniOptional<z.ZodMiniPipe<z.core.$ZodBranded<z.ZodMiniNumber<...>, "u53", "out">, z.ZodMiniTransform<... | ..., ... & ...>>>;
			mime: z.ZodMiniOptional<z.ZodMiniString<string>>;
			mode: z.ZodMiniString<string>;
			track: z.ZodMiniOptional<z.ZodMiniString<string>>;
		}, z.core.$loose>>;
	};
	clock?: {
		timescale: number;
		wall: number & $brand<"u53">;
	};
	echo?: {
		path: RelativeBroadcast;
	};
	json?: {
		tracks: z.ZodMiniRecord<z.ZodMiniString<string>, z.ZodMiniObject<{
			bitrate: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>;
			broadcast: z.ZodMiniOptional<z.ZodMiniType<RelativeBroadcast, string, z.core.$ZodTypeInternals<RelativeBroadcast, string>>>;
			compression: z.ZodMiniOptional<z.ZodMiniString<string>>;
			delay: z.ZodMiniOptional<z.ZodMiniPipe<z.core.$ZodBranded<z.ZodMiniNumber<...>, "u53", "out">, z.ZodMiniTransform<... | ..., ... & ...>>>;
			epoch: z.ZodMiniOptional<z.ZodMiniType<ReferenceEpoch, string, z.core.$ZodTypeInternals<ReferenceEpoch, string>>>;
			jitter: z.ZodMiniOptional<z.ZodMiniPipe<z.core.$ZodBranded<z.ZodMiniNumber<...>, "u53", "out">, z.ZodMiniTransform<... | ..., ... & ...>>>;
			mode: z.ZodMiniString<string>;
			schema: z.ZodMiniOptional<z.ZodMiniString<string>>;
			track: z.ZodMiniOptional<z.ZodMiniString<string>>;
		}, z.core.$loose>>;
	};
	stats?: {
		track: string;
	};
	text?: {
		renditions: z.ZodMiniRecord<z.ZodMiniString<string>, z.ZodMiniObject<{
			broadcast: z.ZodMiniOptional<z.ZodMiniType<RelativeBroadcast, string, z.core.$ZodTypeInternals<RelativeBroadcast, string>>>;
			container: z.ZodMiniDefault<z.ZodMiniUnion<readonly [..., ...]>>;
			delay: z.ZodMiniOptional<z.ZodMiniPipe<z.core.$ZodBranded<z.ZodMiniNumber<...>, "u53", "out">, z.ZodMiniTransform<... | ..., ... & ...>>>;
			epoch: z.ZodMiniOptional<z.ZodMiniType<ReferenceEpoch, string, z.core.$ZodTypeInternals<ReferenceEpoch, string>>>;
			format: z.ZodMiniString<string>;
			jitter: z.ZodMiniOptional<z.ZodMiniPipe<z.core.$ZodBranded<z.ZodMiniNumber<...>, "u53", "out">, z.ZodMiniTransform<... | ..., ... & ...>>>;
			label: z.ZodMiniOptional<z.ZodMiniString<string>>;
			lang: z.ZodMiniOptional<z.ZodMiniString<string>>;
			role: z.ZodMiniDefault<z.ZodMiniString<string>>;
			track: z.ZodMiniOptional<z.ZodMiniString<string>>;
		}, z.core.$strip>>;
	};
	version?: number;
	video?: {
		display: {
			height: number & $brand<"u53">;
			width: number & $brand<"u53">;
		} | undefined;
		flip: undefined;
		renditions: {
			[k: string]: {
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
							x: ...;
							y: ...;
						};
						green: {
							x: ...;
							y: ...;
						};
						maxLuminance: number;
						minLuminance: number;
						red: {
							x: ...;
							y: ...;
						};
						white: {
							x: ...;
							y: ...;
						};
					};
					matrix?: number & $brand<...>;
					primaries?: number & $brand<...>;
					range?: "full" | "limited";
					transfer?: number & $brand<...>;
				};
				container: {
					kind: "legacy";
				} | {
					init: string;
					kind: "cmaf";
				} | {
					kind: "loc";
				} | {
					kind: "unknown";
					raw: {
						[x: string]: unknown;
						kind: string;
					};
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
					kind: "rectilinear";
				} | {
					fov?: number;
					kind: "equirectangular";
				} | {
					kind: "unknown";
					raw: {
						[x: string]: unknown;
						kind: string;
					};
				};
				reorder?: number & $brand<"u8">;
				stereo?: string;
				track?: string;
				warmup?: number & $brand<"u53">;
			};
		};
		rotation: undefined;
	} | {
		display?: {
			height: number & $brand<"u53">;
			width: number & $brand<"u53">;
		};
		flip?: boolean;
		renditions: z.ZodMiniRecord<z.ZodMiniString<string>, z.ZodMiniObject<{
			bitrate: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>;
			broadcast: z.ZodMiniOptional<z.ZodMiniType<RelativeBroadcast, string, z.core.$ZodTypeInternals<RelativeBroadcast, string>>>;
			codec: z.ZodMiniString<string>;
			codedHeight: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>;
			codedWidth: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>;
			color: z.ZodMiniOptional<z.ZodMiniObject<{
				contentLight: z.ZodMiniOptional<...>;
				mastering: z.ZodMiniOptional<...>;
				matrix: z.ZodMiniOptional<...>;
				primaries: z.ZodMiniOptional<...>;
				range: z.ZodMiniOptional<...>;
				transfer: z.ZodMiniOptional<...>;
			}, z.core.$strip>>;
			container: z.ZodMiniDefault<z.ZodMiniUnion<readonly [..., ...]>>;
			delay: z.ZodMiniOptional<z.ZodMiniPipe<z.core.$ZodBranded<z.ZodMiniNumber<...>, "u53", "out">, z.ZodMiniTransform<... | ..., ... & ...>>>;
			description: z.ZodMiniOptional<z.ZodMiniString<string>>;
			displayAspectHeight: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>;
			displayAspectWidth: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>;
			enabled: z.ZodMiniOptional<z.ZodMiniBoolean<boolean>>;
			epoch: z.ZodMiniOptional<z.ZodMiniType<ReferenceEpoch, string, z.core.$ZodTypeInternals<ReferenceEpoch, string>>>;
			framerate: z.ZodMiniOptional<z.ZodMiniNumber<number>>;
			interlaced: z.ZodMiniOptional<z.ZodMiniBoolean<boolean>>;
			jitter: z.ZodMiniOptional<z.ZodMiniPipe<z.core.$ZodBranded<z.ZodMiniNumber<...>, "u53", "out">, z.ZodMiniTransform<... | ..., ... & ...>>>;
			label: z.ZodMiniOptional<z.ZodMiniString<string>>;
			optimizeForLatency: z.ZodMiniOptional<z.ZodMiniBoolean<boolean>>;
			preference: z.ZodMiniOptional<z.ZodMiniInt32>;
			projection: z.ZodMiniOptional<z.ZodMiniUnion<readonly [..., ...]>>;
			reorder: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u8", "out">>;
			stereo: z.ZodMiniOptional<z.ZodMiniString<string>>;
			track: z.ZodMiniOptional<z.ZodMiniString<string>>;
			warmup: z.ZodMiniOptional<z.core.$ZodBranded<z.ZodMiniNumber<number>, "u53", "out">>;
		}, z.core.$strip>>;
		rotation?: number;
	};
}>;
```
