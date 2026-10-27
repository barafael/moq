// Emit declaration-only snapshots into scratch, so generating docs never races a package's
// normal dist build. API Extractor's shared configuration remains the only doc-model source.
import { execFileSync } from "node:child_process";
import { cpSync, existsSync, mkdirSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { writeModels } from "../../js/common/api";

const root = resolve(dirname(fileURLToPath(import.meta.url)), "../..");
const output = join(root, ".tmp/api-reference");
const workspace = JSON.parse(readFileSync(join(root, "package.json"), "utf8"));
rmSync(output, { recursive: true, force: true });
mkdirSync(output, { recursive: true });
const entries: { package: string; entry: string; file: string }[] = [];
for (const directory of workspace.workspaces as string[]) {
	const source = join(root, directory);
	const manifest = JSON.parse(readFileSync(join(source, "package.json"), "utf8"));
	if (!manifest.name.startsWith("@moq/") || !manifest.scripts?.release) continue;
	const slug = manifest.name.slice("@moq/".length);
	const declarations = join(output, "declarations", slug);
	mkdirSync(declarations, { recursive: true });
	const project = join(source, "tsconfig.build.json");
	if (existsSync(project)) {
		execFileSync(
			"bunx",
			[
				"tsc",
				"--project",
				project,
				"--outDir",
				declarations,
				"--noEmit",
				"false",
				"--emitDeclarationOnly",
				"--declarationMap",
				"false",
				"--sourceMap",
				"false",
				"--inlineSources",
				"false",
			],
			{ cwd: root, stdio: "inherit" },
		);
	}
	const exports: Record<string, { types: string }> = {};
	for (const [subpath, target] of Object.entries(manifest.exports)) {
		const types = typeof target === "string" ? target : (target as { types?: string }).types;
		if (!types) throw new Error(`${manifest.name}${subpath}: no declaration source`);
		const emitted = types.endsWith(".d.ts") ? types : types.replace(/^\.\/src\//, "./").replace(/\.tsx?$/, ".d.ts");
		if (!existsSync(project)) {
			mkdirSync(dirname(join(declarations, emitted)), { recursive: true });
			cpSync(join(source, types), join(declarations, emitted));
		}
		exports[subpath] = { types: emitted };
		const entry = subpath === "." ? "index" : subpath.replace(/^\.\//, "").replaceAll("/", "-");
		entries.push({ package: manifest.name, entry, file: join(output, "models", slug, `${entry}.api.json`) });
	}
	writeFileSync(join(declarations, "package.json"), JSON.stringify({ ...manifest, exports }));
	writeModels(source, join(output, "models", slug), declarations);
	rmSync(declarations, { recursive: true, force: true });
}
writeFileSync(join(output, "index.json"), JSON.stringify(entries));
console.log(`API reference: ${entries.length} entrypoints from current declarations`);
