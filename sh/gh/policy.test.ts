// PRs validate independently; release PRs also require the full Release gate.
import assert from "node:assert/strict";
import { readdir } from "node:fs/promises";

const workflow = async (name: string) => Bun.YAML.parse(await Bun.file(`.github/workflows/${name}.yml`).text());
const check = await workflow("check");
const gate = await workflow("release-gate");
const platform = await workflow("platform");
const dart = await workflow("release-dart-ffi");
const nightly = await workflow("nightly");

function condition(expression: string, event: string, base: string, selected = true, action = "opened"): unknown {
	const source = expression.replace(/^\$\{\{\s*|\s*\}\}$/g, "");
	const github = { event_name: event, base_ref: base, event: { action } };
	const outputs = Object.fromEntries(
		["needed", "rs", "swift", "cpp", "obs", "moq_sh"].map((key) => [key, String(selected)]),
	);
	return new Function("github", "needs", `return (${source});`)(github, { scope: { outputs } });
}

// Required PR checks always report, including retargeted PRs and empty diffs.
assert.equal(check.on.merge_group, undefined);
assert.equal(check.on.pull_request.paths, undefined, "required PR contexts must always report");
assert.equal(check.on.pull_request.branches, undefined, "all PRs must report the required names");
assert.ok(check.on.pull_request.types.includes("edited"), "retargeting must rerun");
for (const base of ["main", "release"]) {
	assert.equal(condition(check.jobs.check.if, "pull_request", base), true);
	assert.equal(condition(check.jobs.check.if, "pull_request", base, true, "edited"), true);
	assert.equal(condition(check.jobs.check.if, "pull_request", base, true, "closed"), false);
}
// Scope selection and execution must use the same PR base, even when main advances.
const scoped = check.jobs.check.steps.filter((step: { env?: { CI_BASE?: string } }) => step.env?.CI_BASE);
assert.equal(scoped.length, 2);
for (const step of scoped) {
	const expression = step.env.CI_BASE.replace(/^\$\{\{\s*|\s*\}\}$/g, "");
	const base = new Function("github", `return (${expression});`);
	assert.equal(base({ base_ref: "main", event: { pull_request: { base: { sha: "pr-base" } } } }), "pr-base");
	assert.equal(base({ base_ref: "release" }), "--all");
}
// Merged PRs use the base ref, but must still cancel the same PR's open run.
for (const config of [check, gate]) {
	const group = (number: number, ref: string) =>
		config.concurrency.group.replace(/\$\{\{(.+?)\}\}/g, (_: string, expression: string) =>
			new Function("github", `return (${expression});`)({ ref, event: { pull_request: { number } } }),
		);
	assert.equal(group(1, "refs/pull/1/merge"), group(1, "refs/heads/main"));
	assert.notEqual(group(1, "refs/heads/main"), group(2, "refs/heads/main"));
	assert.equal(config.concurrency["cancel-in-progress"], true);
}

// Every other workflow stays out of main PRs: optional
// checks run nightly on main, and on release PRs through the Release gate.
const files = (await readdir(".github/workflows")).filter((file) => file.endsWith(".yml"));
for (const file of files) {
	const name = file.replace(/\.yml$/, "");
	if (name === "check" || name === "release-gate") continue;
	const config = await workflow(name);
	assert.equal(config.on.merge_group, undefined, `${file} must not run in the merge queue`);
	assert.equal(config.on.pull_request, undefined, `${file} must run on release PRs through release-gate.yml`);
}

// Release PRs report one context for every optional workflow, retargeted PRs included.
assert.deepEqual(Object.keys(gate.on), ["pull_request"]);
assert.deepEqual(gate.on.pull_request.branches, ["release"]);
assert.equal(gate.on.pull_request.paths, undefined, "the required release PR context must always report");
assert.ok(gate.on.pull_request.types.includes("edited"), "retargeting to release must rerun");
assert.ok(gate.on.pull_request.types.includes("closed"), "closing must cancel an in-flight run");

const calls = Object.entries(gate.jobs).filter(([, job]) => (job as { uses?: string }).uses) as [
	string,
	{ uses: string; if: string },
][];
for (const [id, job] of calls) {
	const called = await workflow(job.uses.replace(/^\.\/\.github\/workflows\/|\.yml$/g, ""));
	assert.ok("workflow_call" in called.on, `${job.uses} must be callable`);
	assert.equal(
		id,
		job.uses.replace(/^\.\/\.github\/workflows\/|\.yml$/g, ""),
		`${id} must be named after its workflow`,
	);
}
// Every workflow that used to run on release PRs blocks a release now.
assert.deepEqual(calls.map(([id]) => id).sort(), [
	"android",
	"audio-quality",
	"cpp",
	"features",
	"interop",
	"moq-sh",
	"obs",
	"oss-scanner",
	"platform",
	"release-dart",
	"release-dart-ffi",
	"release-go-ffi",
	"release-js",
	"release-kt-lib",
	"release-swift-lib",
	"runner-vm",
	"ts",
	"uring",
	"video",
	"wasm",
	"x11",
]);

// Platform and Features select their own work; every other call runs when its filter matches.
const filters = Object.keys(Bun.YAML.parse(gate.jobs.scope.steps.at(-1).with.filters) as object);
const unfiltered = ["features", "platform"];
for (const [id, job] of calls) {
	if (unfiltered.includes(id)) {
		assert.equal(condition(job.if, "pull_request", "release"), true);
		assert.equal(condition(job.if, "pull_request", "release", true, "closed"), false);
		continue;
	}
	assert.equal(job.if, `needs.scope.outputs.${id} == 'true'`);
	assert.equal(gate.jobs.scope.outputs[id], `\${{ steps.filter.outputs.${id} }}`);
}
assert.deepEqual(
	filters.sort(),
	calls
		.map(([id]) => id)
		.filter((id) => !unfiltered.includes(id))
		.sort(),
);
assert.deepEqual(Object.keys(gate.jobs.scope.outputs).sort(), filters.sort());
// The REST file list stops at 3,000 files, so the filter diffs with git.
assert.equal(gate.jobs.scope.steps.at(-1).with.token, "");
assert.equal(gate.jobs.scope.steps[0].with["fetch-depth"], 0);

// The gate fails on any failure or cancellation, a failed scope included, and
// passes skipped workflows. It runs even in a cancelled run, so an `edited`
// rerun never leaves a skipped, passing context on the commit.
const release = gate.jobs.gate;
assert.equal(release.name, "Release gate");
assert.deepEqual(
	[...release.needs].sort(),
	Object.keys(gate.jobs)
		.filter((id) => id !== "gate")
		.sort(),
);
function verdict(expression: string, action: string, results: string[]): unknown {
	const source = expression
		.replace(/always\(\)/g, "true")
		.replace(/contains\(needs\.\*\.result, '(\w+)'\)/g, "results.includes('$1')");
	return new Function("github", "results", `return (${source});`)({ event: { action } }, results);
}
assert.equal(verdict(release.if, "synchronize", ["cancelled"]), true);
assert.equal(verdict(release.if, "closed", ["success"]), false);
const fail = release.steps.find((step: { run?: string }) => step.run === "exit 1");
for (const [results, failed] of [
	[["success", "skipped"], false],
	[["success", "failure"], true],
	[["skipped", "cancelled"], true],
] as const) {
	assert.equal(verdict(fail.if, "synchronize", [...results]), failed, `gate over ${results}`);
}

// Platform selects its native suites from the diff, and Nightly runs it on main.
assert.deepEqual(Object.keys(platform.jobs).sort(), ["macos", "scope", "windows"]);
// The selector runs before any toolchain, through the same query as Check.
assert.equal(platform.jobs.scope.steps.at(-1).run, 'just ci-select-platform "$CI_BASE" >> "$GITHUB_OUTPUT"');
for (const job of [platform.jobs.macos, platform.jobs.windows]) {
	assert.equal(condition(job.if, "pull_request", "release", false), false);
	assert.equal(condition(job.if, "pull_request", "release", true), true);
}
assert.equal(nightly.jobs.platform.uses, "./.github/workflows/platform.yml");
assert.deepEqual(platform.on.push.branches, ["release"]);

// Nightly covers what main PRs skip.
for (const name of ["x11", "android", "cpp", "wasm", "audio-quality", "oss-scanner", "runner-vm", "features"]) {
	assert.ok(
		Object.values(nightly.jobs).some(
			(job) => (job as { uses?: string }).uses === `./.github/workflows/${name}.yml`,
		),
		`nightly must call ${name}.yml`,
	);
}
for (const job of ["release-swift-lib", "release-dart-ffi", "release-dart", "release-js", "release-kt-lib"]) {
	assert.ok(nightly.jobs[job], `nightly must dry-run ${job}`);
}

assert.deepEqual((await workflow("obs")).on.push.branches, ["release"]);

// Release Dart FFI only runs on release PRs and nightly, so it packages every platform.
const targets = JSON.parse(dart.jobs.ffi.with.targets);
assert.ok(targets.some((target: string) => target.includes("-apple-")));
assert.ok(targets.some((target: string) => target.includes("-windows-")));

// Neither the feature matrix nor the optimized WASM build shares Check's
// default-feature test artifacts, but both must block a release.
const features = await workflow("features");
assert.deepEqual(features.jobs.features.strategy.matrix.recipe, ["rs features", "js wasm"]);
console.log("CI event policy passed");
