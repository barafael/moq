# `@moq/watch/element`

```ts
export default class extends HTMLElement {
	constructor();
	get announced(): boolean;
	set announced(value: boolean);
	attributeChangedCallback(name: "delay" | "buffer" | "url" | "name" | "announced" | "volume" | "muted" | "paused" | "visible" | "backend" | "versions" | "catalog-format" | "captions", oldValue: string | null, newValue: string | null): void;
	audio: Decoder;
	get backend(): Backend;
	set backend(value: Backend);
	broadcast: Broadcast;
	get buffer(): Milli;
	set buffer(value: Milli);
	get captions(): string | undefined;
	set captions(value: string | undefined);
	get catalog(): {
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
	} | undefined;
	set catalog(value: {
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
	} | undefined);
	get catalogFormat(): "hang" | "msf" | "hangz" | "manual" | undefined;
	set catalogFormat(value: "hang" | "msf" | "hangz" | "manual" | undefined);
	connectedCallback(): void;
	connection: Connection;
	readonly controls: {
		audio: {
			muted: Signal<boolean>;
			target: Signal<Target | undefined>;
			volume: Signal<number>;
		};
		buffer: Signal<Milli>;
		delay: Signal<Delay>;
		paused: Signal<boolean>;
		text: {
			target: Signal<string | undefined>;
		};
		video: {
			backend: Signal<Backend>;
			target: Signal<Target | undefined>;
			visible: Signal<Visible>;
		};
	};
	get delay(): Delay;
	set delay(value: Delay);
	disconnectedCallback(): void;
	emitter: Emitter;
	get muted(): boolean;
	set muted(value: boolean);
	get name(): Valid;
	set name(value: string | Valid);
	static observedAttributes: readonly ["url", "versions", "name", "paused", "volume", "muted", "visible", "backend", "announced", "delay", "buffer", "catalog-format", "captions"];
	get paused(): boolean;
	set paused(value: boolean);
	readonly player: Player;
	renderer: Renderer;
	reset(): void;
	readonly signals: Effect;
	sync: Sync;
	text: Source;
	textRenderer: Renderer;
	get url(): URL | undefined;
	set url(value: string | URL | undefined);
	get versions(): readonly string[] | undefined;
	set versions(value: readonly string[] | undefined);
	video: Decoder;
	get visible(): Visible;
	set visible(value: Visible);
	get volume(): number;
	set volume(value: number);
}

export function parseCatalogFormat(value: string | null): "hang" | "msf" | "hangz" | "manual" | undefined;
```
