# `@moq/boy/element`

```ts
export default class extends HTMLElement {
	constructor();
	attributeChangedCallback(name: "url" | "prefix" | "prefix-game" | "prefix-viewer", _oldValue: string | null, newValue: string | null): void;
	connectedCallback(): void;
	readonly connection: Connection;
	disconnectedCallback(): void;
	readonly expanded: Signal<string | undefined>;
	readonly games: Signal<ReadonlyMap<string, Game>>;
	static observedAttributes: readonly ["url", "prefix", "prefix-game", "prefix-viewer"];
	readonly origin: Producer;
	get prefixGame(): string;
	set prefixGame(value: string);
	get prefixPath(): string;
	set prefixPath(value: string);
	get prefixViewer(): string;
	set prefixViewer(value: string);
	get url(): URL | undefined;
	set url(value: string | URL | undefined);
}
```
