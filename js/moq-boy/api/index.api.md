# `@moq/boy`

```ts
export class Game {
	constructor(config: GameConfig);
	readonly active: Signal<boolean>;
	readonly audioDecoder: Decoder;
	readonly audioEmitter: Emitter;
	readonly audioSource: Source;
	readonly broadcast: Broadcast;
	readonly canvas: Signal<HTMLCanvasElement | undefined>;
	close(): void;
	readonly delay: Signal<Delay>;
	readonly expanded: Signal<string | undefined>;
	readonly heldButtons: Set<string>;
	readonly hovered: Signal<boolean>;
	sendButtons(): void;
	sendCommand(cmd: Record<string, unknown>): void;
	readonly sessionId: string;
	readonly status: Signal<{
		buttons: string[];
		latency: Record<string, {
			label: string;
			ms: number;
		}[]>;
		location?: string;
		stats?: {
			audio_secs: number;
			emulation_secs: number;
			video_secs: number;
			wall_secs: number;
		};
	} | undefined>;
	readonly sync: Sync;
	readonly userMuted: Signal<boolean>;
	readonly videoDecoder: Decoder;
	readonly videoRenderer: Renderer;
	readonly videoSource: Source;
	readonly viewerId: Signal<string | undefined>;
	readonly volume: Signal<number>;
}

export interface GameConfig {
	connection: Connection;
	expanded: Signal<string | undefined>;
	gamePrefix: string;
	origin: Producer;
	sessionId: string;
	viewerPrefix: string;
}

export type GameStats = z.infer<typeof GameStatsSchema>;

export const GameStatsSchema: z.ZodMiniObject<{
	audio_secs: z.ZodMiniNumber<number>;
	emulation_secs: z.ZodMiniNumber<number>;
	video_secs: z.ZodMiniNumber<number>;
	wall_secs: z.ZodMiniNumber<number>;
}, z.core.$strip>;

export type GameStatus = z.infer<typeof GameStatusSchema>;

export const GameStatusSchema: z.ZodMiniObject<{
	buttons: z.ZodMiniArray<z.ZodMiniString<string>>;
	latency: z.ZodMiniRecord<z.ZodMiniString<string>, z.ZodMiniArray<z.ZodMiniObject<{
		label: z.ZodMiniString<string>;
		ms: z.ZodMiniNumber<number>;
	}, z.core.$strip>>>;
	location: z.ZodMiniOptional<z.ZodMiniString<string>>;
	stats: z.ZodMiniOptional<z.ZodMiniObject<{
		audio_secs: z.ZodMiniNumber<number>;
		emulation_secs: z.ZodMiniNumber<number>;
		video_secs: z.ZodMiniNumber<number>;
		wall_secs: z.ZodMiniNumber<number>;
	}, z.core.$strip>>;
}, z.core.$strip>;

export const KEY_MAP: Record<string, string>;

export * as Moq from "@moq/net";

export { default as MoqBoy } from "@moq/boy/element";

export * as Watch from "@moq/watch";
```
