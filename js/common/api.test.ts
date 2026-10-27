import { afterEach, expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import { checkReports, writeModels } from "./api.ts";

const directories: string[] = [];
const previousMode = process.env.MOQ_API_REPORT;

afterEach(() => {
	for (const directory of directories.splice(0)) rmSync(directory, { recursive: true, force: true });
	if (previousMode === undefined) delete process.env.MOQ_API_REPORT;
	else process.env.MOQ_API_REPORT = previousMode;
});

const summary = "/**\n * A package summary.\n * @module\n */\n";

function fixture(files: Record<string, string>, manifest: Record<string, unknown> = {}): string {
	const directory = mkdtempSync(join(tmpdir(), "moq-api-test-"));
	directories.push(directory);
	const all: Record<string, string> = {
		"dist/package.json": JSON.stringify({
			name: "@moq/example",
			version: "0.0.0",
			exports: { ".": { types: "./index.d.ts" } },
			...manifest,
		}),
		...files,
	};
	for (const [path, body] of Object.entries(all)) {
		mkdirSync(dirname(join(directory, path)), { recursive: true });
		writeFileSync(join(directory, path), body);
	}
	process.env.MOQ_API_REPORT = "update";
	return directory;
}

test("a documented declaration produces a report", () => {
	const directory = fixture({
		"dist/index.d.ts": `${summary}/** Read a value. */\nexport declare function read(): string;\n`,
	});
	checkReports(directory);
	expect(readFileSync(join(directory, "api/index.api.md"), "utf8")).toContain("export function read(): string;");
});

test("a declaration needs a summary", () => {
	// The module summary would document a declaration directly after it.
	const directory = fixture({
		"dist/index.d.ts": `${summary}/** Write a value. */\nexport declare function write(): void;\nexport declare function read(): string;\n`,
	});
	expect(() => checkReports(directory)).toThrow("API Extractor failed");
});

test("a signature cannot depend on a declaration no entrypoint exports", () => {
	const directory = fixture({
		"dist/index.d.ts": `${summary}export { read } from "./access";\n`,
		"dist/access.d.ts": `/** Input to a read. */
export interface AccessInput {
	/** The path to read. */
	path: string;
}
/** Read a value. */
export declare function read(input: AccessInput): string;
`,
	});
	expect(() => checkReports(directory)).toThrow("API Extractor failed");
});

test("a signature may use a declaration another entrypoint exports, or one it renames", () => {
	const directory = fixture(
		{
			"dist/index.d.ts": `${summary}export { type AccessInput } from "./access";\n`,
			"dist/access.d.ts": `/** Input to a read. */
export interface AccessInput {
	/** The path to read. */
	path: string;
}
/** Parse an input. */
declare function parse(text: string): AccessInput;
/** Reads values. */
export declare class Reader {
	/** Read a value. */
	read(input: AccessInput): string;
	/** Parse an input. */
	static parse: typeof parse;
}
`,
			"dist/reader.d.ts": `${summary}export { Reader } from "./access";\n`,
		},
		{ exports: { ".": { types: "./index.d.ts" }, "./reader": { types: "./reader.d.ts" } } },
	);
	checkReports(directory);
	expect(readFileSync(join(directory, "api/reader.api.md"), "utf8")).toContain("static parse: typeof parse;");
});

test("a declaration resolves types from declared dependencies only", () => {
	const files = {
		"node_modules/dep/package.json": JSON.stringify({ name: "dep", version: "0.0.0", types: "./index.d.ts" }),
		"node_modules/dep/index.d.ts": "export interface Value {\n\tvalue: string;\n}\n",
		"dist/index.d.ts": `${summary}import type { Value } from "dep";\n/** Read a value. */\nexport declare function read(): Value;\n`,
	};
	checkReports(fixture(files, { dependencies: { dep: "^0.0.0" } }));
	expect(() => checkReports(fixture(files))).toThrow("API Extractor failed");
});

test("a hidden declaration cannot borrow an exported declaration's name", () => {
	const directory = fixture({
		"dist/index.d.ts": `${summary}export { read } from "./access";
/** A public value. */
export interface Value {
/** Its text. */
text: string;
}
`,
		"dist/access.d.ts": `/** A hidden value. */
interface Value {
/** Its count. */
count: number;
}
/** Read the hidden type. */
export declare function read(): Value;
`,
	});
	expect(() => checkReports(directory)).toThrow("API Extractor failed");
});

test("alias chains expose the declaration they name and produce doc models", () => {
	const directory = fixture({
		"dist/index.d.ts": `${summary}/** The underlying value. */
interface Value {
/** Its text. */
text: string;
}
/** The first name. */
export type First = Value;
/** A second name. */
export type Second = First;
/** Read by either name. */
export declare function read(input: Second): Value;
`,
	});
	checkReports(directory);
	const output = join(directory, "models");
	writeModels(directory, output);
	const model = JSON.parse(readFileSync(join(output, "index.api.json"), "utf8"));
	expect(model.name).toBe("@moq/example");
	expect(model.members[0].members.map((item: { name: string }) => item.name)).toContain("Second");
	expect(model.members[0].members.find((item: { name: string }) => item.name === "read").docComment).toContain(
		"Read by either name",
	);
});

// These pairs are counterexamples to recovering public export names from an API model.
// Keep the inputs beside the checkpoint so a newer extractor can prove the limit changed.
test("different public namespace aliases can have indistinguishable doc models", () => {
	const model = (name: string) => {
		const directory = fixture({
			"dist/index.d.ts": `${summary}export * as ${name} from "./errors";\n`,
			"dist/errors.d.ts": "/** Return the builtin error. */\nexport declare function fail(): Error;\n",
		});
		const output = join(directory, "models");
		writeModels(directory, output);
		return JSON.parse(readFileSync(join(output, "index.api.json"), "utf8"));
	};
	const named = model("Error");
	const suffixed = model("Error_2");
	expect(named).toEqual(suffixed);
	const namespace = named.members[0].members.find((item: { kind: string }) => item.kind === "Namespace");
	expect(namespace.name).toBe("Error_2");
	expect(namespace.canonicalReference).toBe("@moq/example!Error_2:namespace");
	expect(namespace.fileUrlPath).toBe("index.d.ts");
	expect(namespace.excerptTokens).toEqual([]);
});

test("different public external re-exports can have indistinguishable empty doc models", () => {
	const model = (alias: string) => {
		const directory = fixture(
			{
				"dist/index.d.ts": `${summary}export { Thing as ${alias} } from "dep";\n`,
				"node_modules/dep/package.json": JSON.stringify({ name: "dep", version: "1.0.0", types: "index.d.ts" }),
				"node_modules/dep/index.d.ts": "/** A dependency's public value. */\nexport declare class Thing {}\n",
			},
			{ dependencies: { dep: "^1.0.0" } },
		);
		const output = join(directory, "models");
		writeModels(directory, output);
		return JSON.parse(readFileSync(join(output, "index.api.json"), "utf8"));
	};
	const first = model("First");
	const second = model("Second");
	expect(first).toEqual(second);
	expect(first.members[0].members).toEqual([]);
});
