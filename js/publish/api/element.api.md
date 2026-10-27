# `@moq/publish/element`

```ts
export type AnnounceMode = "always" | "source";

export type SourceType = "camera" | "screen" | "file";

export default class extends HTMLElement {
	constructor();
	get announce(): AnnounceMode | undefined;
	set announce(value: AnnounceMode);
	attributeChangedCallback(name: "source" | "url" | "name" | "announce" | "backend" | "versions" | "muted" | "invisible" | "preview", oldValue: string | null, newValue: string | null): void;
	audio: Encoder;
	get backend(): Backend;
	set backend(value: Backend);
	broadcast: Broadcast;
	connectedCallback(): void;
	connection: Connection;
	readonly controls: {
		announce: Signal<AnnounceMode | undefined>;
		backend: Signal<Backend>;
		invisible: Signal<boolean>;
		muted: Signal<boolean>;
		preview: Signal<Mode>;
		source: Signal<File | SourceType | undefined>;
	};
	disconnectedCallback(): void;
	get invisible(): boolean;
	set invisible(value: boolean);
	get muted(): boolean;
	set muted(value: boolean);
	get name(): Valid;
	set name(value: string | Valid);
	static observedAttributes: readonly ["url", "versions", "name", "muted", "invisible", "source", "preview", "backend", "announce"];
	get preview(): Mode;
	set preview(value: Mode);
	readonly signals: Effect;
	get source(): File | SourceType | undefined;
	set source(value: File | SourceType | undefined);
	readonly sources: Moq.Signals.Readonlys<{
		audio: Signal<Source.Microphone | Source.Screen | undefined>;
		file: Signal<Source.File | undefined>;
		video: Signal<Source.Camera | Source.Screen | undefined>;
	}>;
	get url(): URL | undefined;
	set url(value: string | URL | undefined);
	get versions(): readonly string[] | undefined;
	set versions(value: readonly string[] | undefined);
	video: Encoder;
}
```
