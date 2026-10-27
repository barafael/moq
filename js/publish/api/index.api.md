# `@moq/publish`

```ts
export namespace Audio {
	export type Aac = "aac" | AacConfig;
	export type AacConfig = {
		bitrate?: number;
		mime: "aac";
	};
	export interface AudioFrame {
		channels: Float32Array<ArrayBufferLike>[];
		timestamp: Micro;
	}
	export class Capture {
		constructor(props?: CaptureProps);
		channelCount: Signal<number | undefined>;
		close(): void;
		readonly in: Readonlys<CaptureInput>;
		readonly out: Readonlys<CaptureOutput>;
		sampleRate: Signal<number | undefined>;
	}
	export type CaptureInput = {
		enabled: Getter<boolean>;
		source: Getter<Source | undefined>;
	};
	export type CaptureOutput = {
		format: Signal<Format | undefined>;
		frames: Signal<Fanout<AudioFrame> | undefined>;
		root: Signal<AudioNode | undefined>;
	};
	export type CaptureProps = Inputs<CaptureInput> & {
		channelCount?: number | Signal<number | undefined>;
		sampleRate?: number | Signal<number | undefined>;
	};
	export type Codec = Opus | Aac;
	export type CodecMime = "opus" | "aac";
	export type Constraints = Omit<MediaTrackConstraints, "aspectRatio" | "backgroundBlur" | "displaySurface" | "facingMode" | "frameRate" | "height" | "width">;
	export class Encoder {
		constructor(name: string, props?: EncoderProps);
		get capture(): Capture | undefined;
		close(): void;
		codec: Signal<Codec>;
		groupDuration: Signal<Milli>;
		readonly in: Readonlys<EncoderInput>;
		readonly name: string;
		readonly out: Readonlys<EncoderOutput>;
		volume: Signal<number>;
	}
	export type EncoderInput = {
		bandwidth: Getter<Moq.Bandwidth.Handle | undefined>;
		broadcast: Getter<Broadcast | undefined>;
		capture: Getter<Capture | undefined>;
		enabled: Getter<boolean>;
	};
	export type EncoderOutput = {
		active: Signal<boolean>;
		catalog: Signal<Catalog.AudioConfig | undefined>;
		error: Signal<Error | undefined>;
		root: Signal<AudioNode | undefined>;
		stats: Signal<Stats>;
	};
	export type EncoderProps = Inputs<EncoderInput> & {
		codec?: Codec | Signal<Codec>;
		groupDuration?: Milli | Signal<Milli>;
		volume?: number | Signal<number>;
	};
	export interface Format {
		channelCount: number;
		sampleRate: number;
	}
	export type Kind = "voice" | "music" | "auto";
	export type Opus = "opus" | OpusConfig;
	export type OpusConfig = {
		bitrate?: number;
		complexity?: number;
		frameDuration?: Milli;
		mime: "opus";
		packetlossperc?: number;
		useinbandfec?: boolean;
	};
	export type Rendition = Rendition<Catalog.AudioConfig>;
	export interface SampleSource {
		channelCount: number;
		kind: Kind;
		sampleRate: number;
		samples: ReadableStream<AudioData>;
	}
	export type Source = StreamTrack | SourceConfig | SampleSource;
	export interface SourceConfig {
		kind: Kind;
		track: StreamTrack;
	}
	export interface Stats {
		bytes: number;
		frames: number;
	}
	export interface StreamTrack extends MediaStreamTrack {
		clone(): StreamTrack;
		getSettings(): TrackSettings;
		kind: "audio";
	}
	export interface TrackSettings {
		autoGainControl?: boolean;
		channelCount?: number;
		deviceId: string;
		echoCancellation?: boolean;
		groupId: string;
		noiseSuppression?: boolean;
		sampleRate: number;
		sampleSize?: number;
	}
	export function isSampleSource(source: Source): source is SampleSource;
	export function normalizeSource(source: StreamTrack | SourceConfig): SourceConfig;
	export function sourceKind(source: Source): Kind;
}

export class Broadcast {
	constructor(props?: Inputs<BroadcastInput>);
	static readonly CATALOG_TRACK: "catalog.json";
	static readonly CATALOG_TRACK_COMPRESSED: "catalog.json.z";
	audio(name: string): Rendition<{
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
	}>;
	readonly catalog: CatalogProducer;
	close(): void;
	readonly in: Readonlys<BroadcastInput>;
	readonly net: Getter<Producer | undefined>;
	text(name: string): Rendition<{
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
	}>;
	video(name: string): Rendition<{
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
	}>;
}

export type BroadcastInput = {
	announce: Getter<boolean>;
	display: Getter<{
		height: number;
		width: number;
	} | undefined>;
	enabled: Getter<boolean>;
	flip: Getter<boolean>;
	maxAge: Getter<Milli | undefined>;
	name: Getter<Moq.Path.Valid>;
	origin: Getter<Moq.Origin.Table | undefined>;
	rotation: Getter<number>;
};

export class CatalogProducer {
	constructor();
	mutate(fn: (catalog: {
		[key: string]: unknown;
		archive?: {
			replay?: RelativeBroadcast;
			replayEpoch?: ReferenceEpoch;
			store?: string;
			timelines: Record<string, {
				duration?: number & $brand<"u53">;
				track: string;
			}>;
			timescale: number & $brand<"u53">;
			version?: number & $brand<"u53">;
		};
		audio?: {
			renditions: {
				[key: string]: {
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
				};
			};
		} | {
			renditions: Record<string, {
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
			}>;
		};
		binary?: {
			tracks: Record<string, {
				[key: string]: unknown;
				bitrate?: number & $brand<"u53">;
				broadcast?: RelativeBroadcast;
				compression?: string;
				delay?: number & $brand<"u53">;
				epoch?: ReferenceEpoch;
				jitter?: number & $brand<"u53">;
				mime?: string;
				mode: string;
				track?: string;
			}>;
		};
		clock?: {
			timescale: number;
			wall: number & $brand<"u53">;
		};
		echo?: {
			path: RelativeBroadcast;
		};
		json?: {
			tracks: Record<string, {
				[key: string]: unknown;
				bitrate?: number & $brand<"u53">;
				broadcast?: RelativeBroadcast;
				compression?: string;
				delay?: number & $brand<"u53">;
				epoch?: ReferenceEpoch;
				jitter?: number & $brand<"u53">;
				mode: string;
				schema?: string;
				track?: string;
			}>;
		};
		stats?: {
			track: string;
		};
		text?: {
			renditions: Record<string, {
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
			}>;
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
						range?: "limited" | "full";
						transfer?: number & $brand<...>;
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
				};
			};
			rotation: undefined;
		} | {
			display?: {
				height: number & $brand<"u53">;
				width: number & $brand<"u53">;
			};
			flip?: boolean;
			renditions: Record<string, {
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
					range?: "limited" | "full";
					transfer?: number & $brand<...>;
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
			}>;
			rotation?: number;
		};
	}) => void): void;
	serve(track: Producer, effect: Effect, opts?: {
		compression?: boolean;
	}): void;
}

export * as Clock from "@moq/clock";

export class Fanout<T> {
	constructor(source: ReadableStream<T>, props?: FanoutProps<T>);
	close(): void;
	subscribe(effect: Effect, queue?: number): ReadableStream<T>;
}

export interface FanoutProps<T> {
	clone?: (value: T) => T;
	queue?: number;
	release?: (value: T) => void;
}

export * as Hang from "@moq/hang";

export type Kind = "video" | "audio" | "text";

export * as Net from "@moq/net";

export namespace Preview {
	export { Backend } from "@moq/video";
	export type Mode = "none" | "source" | "encoded";
	export class Renderer {
		constructor(props?: Inputs<RendererInput>);
		close(): void;
		readonly in: Readonlys<RendererInput>;
		readonly out: Readonlys<{
			error: Getter<Error | undefined>;
		}>;
	}
	export type RendererInput = {
		backend: Getter<Video.Backend>;
		canvas: Getter<HTMLCanvasElement | undefined>;
		display: Getter<{
			height: number;
			width: number;
		} | undefined>;
		enabled: Getter<boolean>;
		encoder: Getter<Encoder.Encoder | undefined>;
		flip: Getter<boolean>;
		frames: Getter<Fanout<VideoFrame> | undefined>;
		mode: Getter<Mode>;
		rotation: Getter<number>;
	};
	export type RendererProps = Inputs<RendererInput>;
	export class Transcode {
		constructor(props?: Inputs<TranscodeInput>);
		close(): void;
		readonly in: Readonlys<TranscodeInput>;
		readonly out: Readonlys<TranscodeOutput>;
	}
	export type TranscodeInput = {
		config: Getter<VideoEncoderConfig | undefined>;
		settings: Getter<Encoder.Config | undefined>;
		source: Getter<Fanout<VideoFrame> | undefined>;
	};
	export type TranscodeOutput = {
		frame: Signal<VideoFrame | undefined>;
	};
	export type TranscodeProps = Inputs<TranscodeInput>;
}

export class Rendition<Config> {
	constructor();
	close(): void;
	readonly config: Signal<Config | undefined>;
	readonly kind: Kind;
	readonly name: string;
	readonly track: Getter<Producer | undefined>;
}

export * as Signals from "@moq/signals";

export namespace Source {
	export class Camera {
		constructor(props?: CameraProps);
		static readonly DEFAULT_CONSTRAINTS: Constraints;
		close(): void;
		constraints: Signal<Constraints | undefined>;
		readonly device: Device<"video">;
		readonly in: Readonlys<CameraInput>;
		readonly out: Readonlys<CameraOutput>;
	}
	export type CameraInput = {
		enabled: Getter<boolean>;
	};
	export type CameraOutput = {
		error: Signal<Error | undefined>;
		orientation: Signal<number>;
		source: Signal<Media | undefined>;
	};
	export interface CameraProps extends Inputs<CameraInput> {
		constraints?: Constraints | Signal<Constraints | undefined>;
		device?: DeviceProps;
	}
	export class Device<Kind extends "audio" | "video"> {
		constructor(kind: Kind, props?: DeviceProps);
		capture(deviceId: string | undefined): Dispose;
		close(): void;
		readonly in: Readonlys<DeviceInput>;
		readonly kind: Kind;
		readonly out: Readonlys<DeviceOutput>;
		preferred: Signal<string | undefined>;
		requestPermission(): void;
	}
	export type DeviceInput = {
		enabled: Getter<boolean>;
	};
	export type DeviceOutput = {
		active: Signal<string | undefined>;
		available: Signal<MediaDeviceInfo[] | undefined>;
		default: Signal<string | undefined>;
		permission: Signal<boolean>;
		requested: Signal<string | undefined>;
	};
	export interface DeviceProps extends Inputs<DeviceInput> {
		preferred?: string | Signal<string | undefined>;
	}
	export class File {
		constructor(props?: FileProps);
		close(): void;
		file: Signal<File | undefined>;
		readonly in: Readonlys<FileInput>;
		readonly out: Readonlys<FileOutput>;
		prompt(): void;
	}
	export type FileInput = {
		enabled: Getter<boolean>;
	};
	export type FileOutput = {
		source: Signal<Media | undefined>;
	};
	export interface FileProps extends Inputs<FileInput> {
		file?: File | Signal<File | undefined>;
	}
	export type Media = {
		audio?: Audio.Source;
		video?: Video.Source;
	};
	export class Microphone {
		constructor(props?: MicrophoneProps);
		close(): void;
		constraints: Signal<Constraints | undefined>;
		readonly device: Device<"audio">;
		readonly in: Readonlys<MicrophoneInput>;
		readonly out: Readonlys<MicrophoneOutput>;
	}
	export type MicrophoneInput = {
		enabled: Getter<boolean>;
	};
	export type MicrophoneOutput = {
		error: Signal<Error | undefined>;
		source: Signal<Media | undefined>;
	};
	export interface MicrophoneProps extends Inputs<MicrophoneInput> {
		constraints?: Constraints | Signal<Constraints | undefined>;
		device?: DeviceProps;
	}
	export class Screen {
		constructor(props?: ScreenProps);
		audio: Signal<boolean | Constraints | undefined>;
		close(): void;
		readonly in: Readonlys<ScreenInput>;
		readonly out: Readonlys<ScreenOutput>;
		video: Signal<boolean | Constraints | undefined>;
	}
	export type ScreenInput = {
		enabled: Getter<boolean>;
	};
	export type ScreenOutput = {
		source: Signal<Media | undefined>;
	};
	export interface ScreenProps extends Inputs<ScreenInput> {
		audio?: boolean | Constraints | Signal<boolean | Constraints | undefined>;
		video?: boolean | Constraints | Signal<boolean | Constraints | undefined>;
	}
}

export namespace Video {
	export class Capture {
		constructor(props?: Inputs<CaptureInput>);
		close(): void;
		readonly in: Readonlys<CaptureInput>;
		readonly out: Readonlys<CaptureOutput>;
	}
	export type CaptureInput = {
		orientation: Getter<number>;
		source: Getter<Source | undefined>;
	};
	export type CaptureOutput = {
		display: Signal<{
			height: number;
			scale?: number;
			width: number;
		} | undefined>;
		frames: Signal<Fanout<VideoFrame> | undefined>;
		rotation: Signal<number>;
	};
	export interface Config {
		bitrateScale?: number;
		codec?: string;
		frameRate?: number;
		keyframeInterval?: Milli;
		maxBitrate?: number;
		maxPixels?: number;
		maxScale?: number;
	}
	export type Constraints = Omit<MediaTrackConstraints, "autoGainControl" | "channelCount" | "echoCancellation" | "noiseSuppression" | "sampleRate" | "sampleSize"> & {
		resizeMode?: "none" | "crop-and-scale";
	};
	export class Encoder {
		constructor(name: string, props?: EncoderProps);
		get capture(): Capture | undefined;
		close(): void;
		config: Signal<Config | undefined>;
		cut(): void;
		readonly in: Readonlys<EncoderInput>;
		readonly name: string;
		readonly out: Readonlys<EncoderOutput>;
	}
	export type EncoderInput = {
		bandwidth: Getter<Moq.Bandwidth.Handle | undefined>;
		broadcast: Getter<Broadcast | undefined>;
		capture: Getter<Capture | undefined>;
		enabled: Getter<boolean>;
	};
	export type EncoderOutput = {
		active: Signal<boolean>;
		catalog: Signal<Catalog.VideoConfig | undefined>;
		error: Signal<Error | undefined>;
		resolved: Signal<VideoEncoderConfig | undefined>;
		stats: Signal<Stats>;
	};
	export type EncoderProps = Inputs<EncoderInput> & {
		config?: Config | Signal<Config | undefined>;
	};
	export interface FrameSource {
		frameRate?: number;
		frames: ReadableStream<VideoFrame>;
	}
	export type Rendition = Rendition<Catalog.VideoConfig>;
	export type Source = StreamTrack | SourceConfig | FrameSource;
	export interface SourceConfig {
		scale?: number;
		track: StreamTrack;
	}
	export interface Stats {
		bytes: number;
		frames: number;
		keyframes: number;
	}
	export interface StreamTrack extends MediaStreamTrack {
		clone(): StreamTrack;
		getSettings(): TrackSettings;
		kind: "video";
	}
	export interface TrackSettings {
		aspectRatio: number;
		deviceId: string;
		facingMode: "user" | "environment" | "left" | "right";
		frameRate?: number;
		groupId: string;
		height: number;
		resizeMode: "none" | "crop-and-scale";
		width: number;
	}
	export function isStreamTrack(source: Source): source is StreamTrack;
	export function normalizeSource(source: StreamTrack | SourceConfig): SourceConfig;
}

export function assets(url: string | URL): void;
```
