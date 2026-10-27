# `@moq/room`

```ts
export namespace Chat {
	export type Event = Json.Window.Event<string>;
	export const HISTORY: Nano;
	export const PRIORITY: 10;
	export class Publisher {
		constructor(track: Producer);
		static create(broadcast: Producer): Publisher;
		expire(): void;
		finish(): void;
		send(text: string): void;
	}
	export class Subscriber {
		constructor(track: Subscriber);
		close(): void;
		recv(): Promise<Timed<Event> | undefined>;
		static subscribe(broadcast: Consumer): Subscriber;
	}
	export const TRACK: "chat";
	export function info(): Pick<Track.Info, "priority" | "timescale">;
}

export type Claims = {
	publish: string[];
	root: string;
	subscribe: string[];
};

export * as Clock from "@moq/clock";

export type Consumed = {
	close: () => void;
	preview: Getter<Preview>;
	user: Readonlys<UserInput>;
};

export type ExtendedCatalog = CatalogRoot & {
	hang?: HangCatalog;
};

export type HangCatalog = {
	chat?: TrackRef;
	location?: TrackRef;
	preview?: TrackRef;
	user?: TrackRef;
};

export const KIND: {
	readonly camera: "camera";
	readonly screen: "screen";
};

export type Kind = typeof KIND[keyof typeof KIND];

export class Local {
	constructor(props: LocalProps);
	readonly camera: Broadcast;
	readonly cameraCapture: Capture;
	readonly cameraEnabled: Signal<boolean>;
	readonly chatting: Signal<boolean>;
	close(): void;
	readonly enabled: Signal<boolean>;
	readonly identity: Getter<Valid>;
	readonly microphone: Microphone;
	readonly microphoneEnabled: Signal<boolean>;
	get preview(): Getter<Preview>;
	readonly screen: Broadcast;
	readonly screenCapture: Capture;
	readonly screenEnabled: Signal<boolean>;
	readonly share: Screen;
	readonly typing: Signal<boolean>;
	readonly user: UserFields;
	readonly webcam: Camera;
}

export interface LocalProps {
	cameraEnabled?: GetterInit<boolean>;
	connection: Connection;
	enabled?: GetterInit<boolean>;
	identity: GetterInit<Valid>;
	microphoneEnabled?: GetterInit<boolean>;
	screenEnabled?: GetterInit<boolean>;
	user?: UserProps;
}

export class Member {
	constructor(kind: Kind, path: Valid, connection: Connection);
	readonly audio: Decoder;
	readonly broadcast: Broadcast;
	readonly canvas: Signal<HTMLCanvasElement | undefined>;
	close(): void;
	readonly emitter: Emitter;
	readonly kind: Kind;
	readonly muted: Signal<boolean>;
	readonly path: Valid;
	readonly player: Player;
	readonly preview: Getter<Preview>;
	readonly renderer: Renderer;
	readonly user: Readonlys<UserInput>;
	readonly video: Decoder;
	readonly volume: Signal<number>;
}

export const PRIORITY: 90;

export type Parsed = {
	identity: Path.Valid;
	kind: Kind;
};

export type Preview = {
	audio?: boolean;
	avatar?: string;
	chat?: boolean;
	name?: string;
	screen?: boolean;
	typing?: boolean;
	video?: boolean;
};

export class Remote {
	constructor(props: RemoteProps);
	attach(kind: Kind, path: Valid): void;
	readonly camera: Getter<Member | undefined>;
	close(): void;
	detach(kind: Kind): void;
	empty(): boolean;
	readonly identity: Valid;
	readonly preview: Getter<Preview>;
	readonly screen: Getter<Member | undefined>;
	readonly user: Readonlys<UserInput>;
}

export interface RemoteProps {
	connection: Connection;
	identity: Valid;
}

export class Room {
	constructor(props: RoomProps);
	close(): void;
	readonly connection: Connection;
	readonly enabled: Getter<boolean>;
	readonly identity: Getter<Valid | undefined>;
	readonly prefix: Getter<Valid | undefined>;
	get remotes(): Getter<Map<Valid, Remote>>;
}

export interface RoomProps {
	connection: Connection;
	enabled?: GetterInit<boolean>;
	identity?: GetterInit<Valid | undefined>;
	prefix?: GetterInit<Valid | undefined>;
}

export const TRACK: {
	readonly chat: "hang/chat.json";
	readonly location: "hang/location.json";
	readonly preview: "hang/preview.json";
	readonly user: "hang/user.json";
};

export type TrackRef = {
	track: string;
};

export type User = {
	avatar?: string;
	color?: string;
	id?: string;
	name?: string;
};

export type UserFields = {
	avatar: Signal<string | undefined>;
	color: Signal<string | undefined>;
	id: Signal<string | undefined>;
	name: Signal<string | undefined>;
};

export type UserInput = {
	avatar: Getter<string | undefined>;
	color: Getter<string | undefined>;
	id: Getter<string | undefined>;
	name: Getter<string | undefined>;
};

export type UserProps = {
	avatar?: string | Signal<string | undefined>;
	color?: string | Signal<string | undefined>;
	id?: string | Signal<string | undefined>;
	name?: string | Signal<string | undefined>;
};

export function broadcastPath(identity: Valid, kind: Kind): Valid;

export function claims(room: string, identity: string): Claims;

export function consume(broadcast: Broadcast): Consumed;

export function isKind(value: string): value is Kind;

export function kindFromSegment(segment: string): Kind | undefined;

export function parse(path: Valid): Parsed | undefined;

export function serve(broadcast: Broadcast, user: UserInput, preview: Getter<Preview>, effect: Effect): void;

export function userFields(props?: UserProps): UserFields;
```
