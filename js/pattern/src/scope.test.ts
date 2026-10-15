import { expect, test } from "bun:test";
import { Pattern, Patterns, Scope, ScopeError } from "./index.ts";

interface Vector {
	include: string[];
	exclude: string[];
	paths: Record<string, boolean>;
	empty: boolean;
}
const vectors = (await Bun.file(
	new URL("../../../rs/moq-pattern/tests/scope.json", import.meta.url),
).json()) as Vector[];
const patterns = (values: string[]) => new Patterns(values.map(Pattern.parse));

test("Rust/JS shared exact scope vectors", () => {
	for (const vector of vectors) {
		const scope = new Scope({ include: patterns(vector.include), exclude: patterns(vector.exclude) });
		expect(scope.isEmpty()).toBe(vector.empty);
		for (const [path, expected] of Object.entries(vector.paths)) {
			expect(scope.matches(path)).toBe(expected);
			expect(scope.rooted("tenant").matches(`tenant/${path}`)).toBe(expected);
		}
	}
});

test("a caller cannot mutate a retained scope through input or output unions", () => {
	const include = patterns(["a/**"]);
	const scope = new Scope({ include });
	include.insert(Pattern.all());
	scope.include.insert(Pattern.all());
	expect(scope.matches("b")).toBe(false);
});

test("complex search refuses instead of dropping denials", () => {
	const long = "a".repeat(Pattern.MAX_INTERSECTIONS + 1);
	const restricted = new Scope({ include: patterns(["**"]), exclude: patterns([long, "b"]) });
	expect(() => restricted.isEmpty()).toThrow(ScopeError);
	expect(restricted.matches(long)).toBe(false);
	expect(restricted.matches("b")).toBe(false);
});

test("exact overlap and coverage agree across a complete small alphabet", () => {
	const paths = [""];
	let level = [""];
	for (let depth = 0; depth < 5; depth++) {
		level = level.flatMap((path) => ["a", "b", "other"].map((part) => (path ? `${path}/${part}` : part)));
		paths.push(...level);
	}
	const scopes = ["", "a", "*", "**", "a/**", "**/a", "a/*", "*/b", "a/**/b"].flatMap((include) =>
		["a", "b/**"].map((exclude) => new Scope({ include: patterns([include]), exclude: patterns([exclude]) })),
	);
	for (const a of scopes) {
		expect(a.isEmpty()).toBe(!paths.some((path) => a.matches(path)));
		for (const b of scopes) {
			expect(a.overlaps(b)).toBe(paths.some((path) => a.matches(path) && b.matches(path)));
			expect(a.covers(b)).toBe(paths.every((path) => !b.matches(path) || a.matches(path)));
			const both = a.intersect(b);
			for (const path of paths) expect(both.matches(path)).toBe(a.matches(path) && b.matches(path));
		}
	}
});
