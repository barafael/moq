# `@moq/signals/react`

```ts
export function useSignal<T>(signal: Signal<T>): [T, (value: T | ((prev: T) => T)) => void];

export function useValue<T>(signal: Getter<T>): T;
```
