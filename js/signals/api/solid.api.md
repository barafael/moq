# `@moq/signals/solid`

```ts
export function createAccessor<T>(signal: Getter<T>): Accessor<T>;

export function createPair<T>(signal: Signal<T>): Signal<T>;

export function createSetter<T>(signal: Signal<T>): Setter<T>;
```
