import { expect, test } from "bun:test";
import { spawnSync } from "node:child_process";
import { mkdtempSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { join, resolve } from "node:path";
import { build } from "vite";
import { workletInline } from "./vite-plugin-worklet";

// A bundler child process would outlive `vite build` and be reaped by init, so its CPU escapes the
// build's accounting and it lingers after the build.
test("builds leave no child process behind", async () => {
	const outDir = mkdtempSync(join(tmpdir(), "worklet-"));
	try {
		await build({
			configFile: false,
			logLevel: "silent",
			plugins: [workletInline()],
			build: {
				outDir,
				lib: { entry: resolve(import.meta.dir, "../watch/src/audio/decoder.ts"), formats: ["es"] },
				rollupOptions: { external: (id) => id.startsWith("@moq/") },
			},
		});
	} finally {
		rmSync(outDir, { recursive: true, force: true });
	}

	expect(spawnSync("pgrep", ["-l", "-P", String(process.pid)], { encoding: "utf8" }).stdout).toBe("");
});
