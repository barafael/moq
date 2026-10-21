/** Time creating and closing an Effect, bare and with a tracked signal, against a bare `new Error()`. */
import { Effect, Signal } from "../src/index.ts";

const runs = 20_000;
const reps = 9;
const signal = new Signal(0);
let checksum = 0;
let tracked = 0;

const ops: [string, () => void | Promise<void>][] = [
	// The creation stack each Effect captures for its diagnostics; the floor for the rows below.
	[
		"new-error",
		() => {
			checksum += new Error("created here:").message.length;
		},
	],
	[
		"create-close",
		() => {
			new Effect().close();
			checksum++;
		},
	],
	[
		"create-get-close",
		async () => {
			const effect = new Effect((effect) => {
				tracked += effect.get(signal) + 1;
			});
			// The callback runs in a microtask, so yield once to let it track the signal before closing.
			await Promise.resolve();
			effect.close();
		},
	],
];

// Nanoseconds per op for the fastest of several reps, since a slower one measured the machine.
async function time(body: () => void | Promise<void>): Promise<number> {
	let best = Number.POSITIVE_INFINITY;
	for (let rep = 0; rep < reps; rep++) {
		const start = performance.now();
		for (let i = 0; i < runs; i++) {
			// Awaiting a sync row would add a microtask hop it doesn't pay.
			const pending = body();
			if (pending) await pending;
		}
		best = Math.min(best, ((performance.now() - start) * 1e6) / runs);
	}
	return best;
}

// Time every op once first and discard it, so the JIT has seen all of them and the first row isn't timing warmup.
for (const [, body] of ops) await time(body);

console.log("op,ns_per_op");
for (const [op, body] of ops) console.log(`${op},${(await time(body)).toFixed(0)}`);
if (checksum === 0) throw new Error("benchmark did no work");
if (tracked === 0) throw new Error("create-get-close never ran its effect");
