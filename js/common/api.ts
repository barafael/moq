// The public API report every `@moq/*` package commits: one `api/*.api.md` per `exports`
// entrypoint, rendered from TypeDoc's model of the built declarations, and diffed against the last
// release by `sh/api-diff`.
//
// The declarations are read from a copy outside the repository whose `node_modules` holds only the
// dependencies its manifest declares, as a consumer's install would, so an undeclared import or a
// broken declaration fails the build. A workspace dependency resolves to its source, so a package's
// report never depends on whether its dependencies are built.
//
// `bun js/common/api.ts baseline <checkout> <out>` renders every published package of another
// checkout, such as the release branch, with this renderer, so `sh/api-diff` compares like with like.

import {
	cpSync,
	existsSync,
	mkdirSync,
	mkdtempSync,
	readdirSync,
	readFileSync,
	realpathSync,
	rmSync,
	symlinkSync,
	writeFileSync,
} from "node:fs";
import { createRequire } from "node:module";
import { tmpdir } from "node:os";
import { dirname, join, relative, resolve } from "node:path";
import {
	Application,
	Converter,
	type DeclarationReflection,
	type ParameterReflection,
	type ProjectReflection,
	ReferenceReflection,
	type Reflection,
	ReflectionKind,
	type SignatureReflection,
	type SomeType,
	type TypeParameterReflection,
} from "typedoc";

/**
 * Compare the built package in `dir` with its committed reports in `api/`, exiting on a
 * difference, or rewrite them when `MOQ_API_REPORT=update`, as `just fix` sets.
 */
export async function checkReports(dir: string) {
	const folder = join(dir, "api");
	const current = await reports(resolve(dir), join(resolve(dir), "dist"), true);
	const committed = existsSync(folder) ? readdirSync(folder).filter((file) => file.endsWith(".api.md")) : [];
	const stale = [...new Set([...current.keys(), ...committed])].filter((file) => {
		const path = join(folder, file);
		return current.get(file) !== (existsSync(path) ? readFileSync(path, "utf8") : undefined);
	});

	if (process.env.MOQ_API_REPORT === "update") {
		mkdirSync(folder, { recursive: true });
		for (const file of stale) {
			const report = current.get(file);
			if (report === undefined) rmSync(join(folder, file));
			else writeFileSync(join(folder, file), report);
		}
	} else if (stale.length > 0) {
		console.error("The public API no longer matches its committed report; run `just fix` and review the diff:");
		for (const file of stale) console.error(`  api/${file}`);
		process.exit(1);
	}
}

interface Manifest {
	name: string;
	exports?: Record<string, string | { types?: string }>;
	scripts?: Record<string, string>;
	dependencies?: Record<string, string>;
	peerDependencies?: Record<string, string>;
	optionalDependencies?: Record<string, string>;
}

/**
 * Every entrypoint's report for the package in `dir`, by file name such as `index.api.md`, from
 * the declarations and manifest in `declarations`. `check` fails on a compiler error in them or a
 * reference to a declaration of this package that no entrypoint exports.
 */
async function reports(dir: string, declarations: string, check: boolean): Promise<Map<string, string>> {
	const work = mkdtempSync(join(tmpdir(), "moq-api-"));
	try {
		const root = join(realpathSync(work), "package");
		cpSync(declarations, root, { recursive: true });
		const manifest: Manifest = JSON.parse(readFileSync(join(root, "package.json"), "utf8"));
		linkDependencies(dir, root, manifest);

		const entries = new Map<string, string>();
		for (const [subpath, target] of Object.entries(manifest.exports ?? {})) {
			const types = typeof target === "string" ? undefined : target.types;
			if (types) entries.set(join(root, types), subpath);
		}
		if (entries.size === 0) return new Map();

		writeFileSync(
			join(root, "tsconfig.json"),
			JSON.stringify({
				compilerOptions: {
					target: "esnext",
					module: "esnext",
					moduleResolution: "bundler",
					lib: ["esnext", "dom"],
					strict: true,
					types: [],
				},
				files: [...entries.keys()],
			}),
		);

		const app = await Application.bootstrap({
			entryPoints: [...entries.keys()],
			entryPointStrategy: "resolve",
			tsconfig: join(root, "tsconfig.json"),
			alwaysCreateEntryPointModule: true,
			readme: "none",
			excludePrivate: true,
			// The report checks its own compiler errors, in this package's files alone.
			skipErrorChecking: true,
			validation: { notExported: false, invalidLink: false, invalidPath: false, rewrittenLink: false },
			// Comments are not part of the API; the reference checks them.
			logLevel: "Error",
		});
		const own = (file: string) => !relative(root, file).startsWith("..");
		const errors: string[] = [];
		if (check) {
			app.converter.on(Converter.EVENT_BEGIN, (context) => {
				for (const program of context.programs) {
					for (const file of program.getSourceFiles()) {
						if (!own(file.fileName)) continue;
						for (const diagnostic of [
							...program.getSyntacticDiagnostics(file),
							...program.getSemanticDiagnostics(file),
						]) {
							const at = diagnostic.start === undefined ? 0 : diagnostic.start;
							const { line } = file.getLineAndCharacterOfPosition(at);
							const text = ts.flattenDiagnosticMessageText(diagnostic.messageText, "\n");
							errors.push(`${relative(root, file.fileName)}:${line + 1}: ${text}`);
						}
					}
				}
			});
		}
		// TypeDoc keeps no record of whether a variable is `const`, so note it from the declaration.
		const constants = new Set<Reflection>();
		app.converter.on(Converter.EVENT_CREATE_DECLARATION, (context, reflection) => {
			const declaration = context.getSymbolFromReflection(reflection)?.valueDeclaration;
			if (
				reflection.kindOf(ReflectionKind.Variable) &&
				declaration &&
				ts.getCombinedNodeFlags(declaration) & ts.NodeFlags.Const
			) {
				constants.add(reflection);
			}
		});
		const project = await app.convert();
		if (!project || app.logger.hasErrors()) {
			throw new Error(`TypeDoc failed on ${manifest.name}`);
		}
		if (check) errors.push(...unexported(app, project, manifest.name));
		if (errors.length > 0) {
			throw new Error(`The declarations of ${manifest.name} are not consumable:\n  ${errors.join("\n  ")}`);
		}

		const modules = new Map<Reflection, string>();
		for (const module of project.children ?? []) {
			const file = module.sources?.[0]?.fullFileName;
			const subpath = file && entries.get(file);
			if (subpath === undefined) throw new Error(`${module.name} is not an entrypoint of ${manifest.name}`);
			modules.set(module, subpath === "." ? manifest.name : `${manifest.name}/${subpath.replace(/^\.\//, "")}`);
		}

		const renderer = new Renderer({ project, name: manifest.name, modules, constants, dir });
		const found = new Map<string, string>();
		for (const [module, specifier] of modules) {
			const subpath = specifier.slice(manifest.name.length + 1) || "index";
			found.set(`${subpath.replaceAll("/", "-")}.api.md`, renderer.report(module as DeclarationReflection));
		}
		return found;
	} finally {
		rmSync(work, { recursive: true, force: true });
	}
}

// The TypeScript TypeDoc compiles with, for its diagnostics and node flags.
const ts: {
	flattenDiagnosticMessageText(text: unknown, newline: string): string;
	getCombinedNodeFlags(node: unknown): number;
	NodeFlags: { Const: number };
} = createRequire(import.meta.resolve("typedoc"))("typescript");

// Every reference to a declaration of this package that the report cannot name, since no
// entrypoint exports it: a consumer could not name it either. Another package's declarations are
// its own report's concern.
function unexported(app: Application, project: ProjectReflection, name: string): string[] {
	const found = new Set<string>();
	const visit = (value: unknown, owner: string) => {
		if (Array.isArray(value)) {
			for (const item of value) visit(item, owner);
		} else if (value && typeof value === "object") {
			const node = value as Record<string, unknown>;
			const target = node.target as { packageName?: string; qualifiedName?: string } | number | undefined;
			const declared = node.type === "reference" && !node.refersToTypeParameter && typeof target === "object";
			if (declared && target.packageName === name) {
				found.add(`${target.qualifiedName}, used by ${owner}, is not exported`);
			}
			const named = node.variant === "declaration" && node.kind !== ReflectionKind.Module;
			const next = named ? (owner ? `${owner}.${node.name}` : String(node.name)) : owner;
			for (const child of Object.values(node)) visit(child, next);
		}
	};
	visit(app.serializer.projectToObject(project, "/"), "");
	return [...found].sort();
}

// Install each declared dependency where the copy resolves it, linked to the workspace's own
// install. Anything undeclared stays unresolvable, as it is for a consumer.
function linkDependencies(dir: string, root: string, manifest: Manifest) {
	const names = new Set(
		(["dependencies", "peerDependencies", "optionalDependencies"] as const).flatMap((field) =>
			Object.keys(manifest[field] ?? {}),
		),
	);
	for (const name of names) {
		const installed = installation(dir, name);
		if (!installed) throw new Error(`${name} is declared but not installed`);
		const link = join(root, "node_modules", name);
		mkdirSync(dirname(link), { recursive: true });
		symlinkSync(realpathSync(installed), link, "dir");
	}
}

// Where Node resolves the package `name` from `dir`.
function installation(dir: string, name: string): string | undefined {
	for (let at = dir; ; at = dirname(at)) {
		const candidate = join(at, "node_modules", name);
		if (existsSync(candidate)) return realpathSync(candidate);
		if (dirname(at) === at) return undefined;
	}
}

// Renders a module as TypeScript declarations, one member per line and sorted by name, so a diff
// shows exactly what changed. Bodies of another package's declarations are left to its own report.
class Renderer {
	#project: ProjectReflection;
	// The package being reported, its entrypoint modules by import specifier, and its `const`s.
	#name: string;
	#modules: Map<Reflection, string>;
	#constants: Set<Reflection>;
	// Where the package is installed, which resolves its dependencies.
	#dir: string;

	constructor(scope: {
		project: ProjectReflection;
		name: string;
		modules: Map<Reflection, string>;
		constants: Set<Reflection>;
		dir: string;
	}) {
		this.#project = scope.project;
		this.#name = scope.name;
		this.#modules = scope.modules;
		this.#constants = scope.constants;
		this.#dir = scope.dir;
	}

	report(module: DeclarationReflection): string {
		const body = this.children(module, "").join("\n\n");
		return `# \`${this.#modules.get(module)}\`\n\n\`\`\`ts\n${body}\n\`\`\`\n`;
	}

	// Each child's declaration, once: a merged class and namespace from another package are two
	// reflections but one re-export.
	private children(container: DeclarationReflection, indent: string): string[] {
		const found = sorted(container.children ?? []).map((child) => this.declaration(child, indent).join("\n"));
		return found.filter((text, i) => text !== found[i - 1]);
	}

	private declaration(item: DeclarationReflection, indent: string): string[] {
		const found = this.named(item, indent);
		if (item.name !== "default") return found;
		// A default export is spelled as the declarations TypeScript emits for it.
		if (item.kind === ReflectionKind.Variable) {
			return [
				`${indent}declare const _default: ${print(item.type, indent)};`,
				`${indent}export default _default;`,
			];
		}
		const [head = "", ...rest] = found;
		return [
			head.replace(/^(\s*)export (abstract class|class|function|interface) default/, "$1export default $2"),
			...rest,
		];
	}

	private named(item: DeclarationReflection, indent: string): string[] {
		const parameters = typeParameters(item.typeParameters, indent);
		const inner = `${indent}\t`;
		const block = (head: string, lines: string[], end = "}") =>
			lines.length ? [`${indent}${head} {`, ...lines, `${indent}${end}`] : [`${indent}${head} {${end}`];
		if (item instanceof ReferenceReflection) return [`${indent}${this.reexport(item)}`];
		const foreign = this.foreign(item);
		if (foreign) return [`${indent}${foreign}`];

		switch (item.kind) {
			case ReflectionKind.Namespace:
				return block(`export namespace ${item.name}`, this.children(item, inner));
			case ReflectionKind.Enum:
				return block(
					`export ${item.flags.isConst ? "const " : ""}enum ${item.name}`,
					sorted(item.children ?? []).map(
						(member) => `${inner}${member.name} = ${print(member.type, inner)},`,
					),
				);
			case ReflectionKind.Class: {
				const heritage =
					clause("extends", item.extendedTypes, indent) + clause("implements", item.implementedTypes, indent);
				const abstract = item.flags.isAbstract ? "abstract " : "";
				return block(`export ${abstract}class ${item.name}${parameters}${heritage}`, lines(item, inner));
			}
			case ReflectionKind.Interface:
				return block(
					`export interface ${item.name}${parameters}${clause("extends", item.extendedTypes, indent)}`,
					lines(item, inner),
				);
			case ReflectionKind.TypeAlias: {
				const literal = item.children ? item : objectLiteral(item.type);
				if (literal) return block(`export type ${item.name}${parameters} =`, lines(literal, inner), "};");
				return [`${indent}export type ${item.name}${parameters} = ${print(item.type, indent)};`];
			}
			case ReflectionKind.Variable: {
				const keyword = this.#constants.has(item) ? "const" : "let";
				return [`${indent}export ${keyword} ${item.name}: ${print(item.type, indent)};`];
			}
			case ReflectionKind.Function:
				return (item.signatures ?? []).map(
					(signature) => `${indent}export function ${item.name}${call(signature, ": ", indent)};`,
				);
			default:
				throw new Error(
					`${item.getFullName()} is a ${ReflectionKind[item.kind]}, which the report cannot render`,
				);
		}
	}

	// A re-export of something this package declares under another name or in another entrypoint.
	private reexport(item: ReferenceReflection): string {
		const target = item.tryGetTargetReflectionDeep();
		if (!target) throw new Error(`${item.getFullName()} re-exports nothing`);
		const foreign = this.foreign(target, item.name);
		if (foreign) return foreign;
		const specifier = this.#modules.get(target);
		if (specifier) return `export * as ${item.name} from "${specifier}";`;
		const module = this.module(target);
		const name = target.getFullName().slice(module.getFullName().length + 1);
		const from = module === this.module(item) ? "" : ` from "${this.#modules.get(module)}"`;
		// A namespace member can only be re-exported as an alias of its qualified name.
		if (!from && name.includes(".")) return `export import ${item.name} = ${name};`;
		return `export { ${alias(name, item.name)} }${from};`;
	}

	// A declaration another package owns, as the re-export that brings it here.
	private foreign(item: Reflection, name = item.name): string | undefined {
		const id = this.#project.getSymbolIdFromReflection(item);
		if (!id || id.packageName === this.#name) return undefined;
		const specifier = this.specifier(id.packageName, id.packagePath);
		// A whole module has no name of its own.
		if (!id.qualifiedName) return `export * as ${name} from "${specifier}";`;
		return `export { ${alias(id.qualifiedName, name)} } from "${specifier}";`;
	}

	// The entrypoint of package `name` whose file is `path`, or the package itself.
	private specifier(name: string, path: string): string {
		const dir = installation(this.#dir, name);
		const manifest: Manifest | undefined = dir && JSON.parse(readFileSync(join(dir, "package.json"), "utf8"));
		for (const [subpath, target] of Object.entries(manifest?.exports ?? {})) {
			const file = typeof target === "string" ? target : target.types;
			if (file?.replace(/^\.\//, "") === path) return subpath === "." ? name : `${name}/${subpath.slice(2)}`;
		}
		return name;
	}

	private module(item: Reflection): Reflection {
		let at = item;
		while (at.parent && !this.#modules.has(at)) at = at.parent;
		return at;
	}
}

// Sorted by name, then kind, so moving a declaration in its file changes nothing.
function sorted<T extends Reflection>(items: T[]): T[] {
	return [...items].sort((a, b) => (a.name === b.name ? a.kind - b.kind : a.name < b.name ? -1 : 1));
}

function alias(name: string, as: string): string {
	return name === as ? name : `${name} as ${as}`;
}

function clause(keyword: string, types: SomeType[] | undefined, indent: string): string {
	return types?.length ? ` ${keyword} ${types.map((type) => print(type, indent)).join(", ")}` : "";
}

// An object type spelled out member by member, rather than a function type or an empty `{}`.
function objectLiteral(type?: SomeType): DeclarationReflection | undefined {
	if (type?.type !== "reflection") return undefined;
	const { declaration } = type;
	return declaration.children || declaration.indexSignatures ? declaration : undefined;
}

function typeParameters(parameters: TypeParameterReflection[] | undefined, indent: string): string {
	if (!parameters?.length) return "";
	const each = parameters.map((parameter) => {
		const modifiers = `${parameter.flags.isConst ? "const " : ""}${parameter.varianceModifier ? `${parameter.varianceModifier} ` : ""}`;
		const constraint = parameter.type ? ` extends ${print(parameter.type, indent)}` : "";
		const fallback = parameter.default ? ` = ${print(parameter.default, indent)}` : "";
		return `${modifiers}${parameter.name}${constraint}${fallback}`;
	});
	return `<${each.join(", ")}>`;
}

function parameter(item: ParameterReflection, indent: string): string {
	// TypeDoc names a destructured parameter `__namedParameters`; its fields are in its type.
	const name = item.name === "__namedParameters" ? "{ ... }" : item.name;
	const optional = item.flags.isOptional || item.defaultValue !== undefined ? "?" : "";
	return `${item.flags.isRest ? "..." : ""}${name}${optional}: ${print(item.type, indent)}`;
}

// A signature's parameters and return type, which `arrow` separates: `: ` for a declaration and
// ` => ` for a function type. A setter has no return type.
function call(signature: SignatureReflection, arrow: string | undefined, indent: string, generic = true): string {
	// A class's type parameters are its own, though TypeDoc repeats them on its constructor.
	const types = generic ? typeParameters(signature.typeParameters, indent) : "";
	const parameters = (signature.parameters ?? []).map((item) => parameter(item, indent)).join(", ");
	const returns = arrow === undefined ? "" : `${arrow}${print(signature.type, indent)}`;
	return `${types}(${parameters})${returns}`;
}

function lines(container: DeclarationReflection, indent: string): string[] {
	return members(container, indent).map((member) => `${indent}${member};`);
}

// A class, interface, or object type's members, each printed at `indent`: signatures and the
// constructor first, then by name.
function members(container: DeclarationReflection, indent: string): string[] {
	const found = [
		...(container.signatures ?? []).map((signature) => call(signature, ": ", indent)),
		...(container.indexSignatures ?? []).map((signature) => {
			const [key] = signature.parameters ?? [];
			return `${modifiers(signature)}[${key?.name}: ${print(key?.type, indent)}]: ${print(signature.type, indent)}`;
		}),
	];
	const rest: string[] = [];
	for (const member of sorted(container.children ?? [])) {
		// Inherited members belong to the declaration they come from.
		if (member.inheritedFrom) continue;
		const prefix = modifiers(member);
		const optional = member.flags.isOptional ? "?" : "";
		switch (member.kind) {
			case ReflectionKind.Constructor:
				for (const signature of member.signatures ?? []) {
					const isClass = container.kind === ReflectionKind.Class;
					const text = isClass
						? `constructor${call(signature, undefined, indent, false)}`
						: `new ${call(signature, ": ", indent)}`;
					found.push(`${prefix}${text}`);
				}
				break;
			case ReflectionKind.Property:
				rest.push(`${prefix}${member.name}${optional}: ${print(member.type, indent)}`);
				break;
			case ReflectionKind.Method:
				for (const signature of member.signatures ?? []) {
					rest.push(`${prefix}${member.name}${optional}${call(signature, ": ", indent)}`);
				}
				break;
			case ReflectionKind.Accessor:
				if (member.getSignature)
					rest.push(`${prefix}get ${member.name}(): ${print(member.getSignature.type, indent)}`);
				if (member.setSignature)
					rest.push(`${prefix}set ${member.name}${call(member.setSignature, undefined, indent)}`);
				break;
			default:
				throw new Error(
					`${member.getFullName()} is a ${ReflectionKind[member.kind]}, which the report cannot render`,
				);
		}
	}
	return [...found, ...rest];
}

function modifiers(item: Reflection): string {
	const { flags } = item;
	return `${flags.isProtected ? "protected " : ""}${flags.isStatic ? "static " : ""}${flags.isAbstract ? "abstract " : ""}${flags.isReadonly ? "readonly " : ""}`;
}

// Where a type is printed, for the parentheses TypeScript's precedence needs there.
type Position = "top" | "union" | "intersection" | "operand" | "check";

// A type as TypeScript spells it. TypeDoc's own `toString` leaves a function type unparenthesized
// in an intersection and drops `readonly` from object members, which would misstate the API.
function print(type: SomeType | undefined, indent: string, at: Position = "top"): string {
	if (!type) return "any";
	const wrap = (text: string, inside: Position[]) => (inside.includes(at) ? `(${text})` : text);
	switch (type.type) {
		case "array":
			return `${print(type.elementType, indent, "operand")}[]`;
		case "conditional":
			return wrap(
				`${print(type.checkType, indent, "check")} extends ${print(type.extendsType, indent, "check")} ? ${print(type.trueType, indent)} : ${print(type.falseType, indent)}`,
				["union", "intersection", "operand", "check"],
			);
		case "indexedAccess":
			return `${print(type.objectType, indent, "operand")}[${print(type.indexType, indent)}]`;
		case "inferred":
			return wrap(
				`infer ${type.name}${type.constraint ? ` extends ${print(type.constraint, indent, "check")}` : ""}`,
				["operand"],
			);
		case "intersection":
			return wrap(type.types.map((member) => print(member, indent, "intersection")).join(" & "), ["operand"]);
		case "union":
			return wrap(type.types.map((member) => print(member, indent, "union")).join(" | "), [
				"intersection",
				"operand",
			]);
		case "intrinsic":
		case "unknown":
			return type.name;
		case "literal":
			return type.toString();
		case "mapped": {
			const readonly = type.readonlyModifier === "-" ? "-readonly " : type.readonlyModifier ? "readonly " : "";
			const optional = type.optionalModifier === "-" ? "-?" : type.optionalModifier ? "?" : "";
			const as = type.nameType ? ` as ${print(type.nameType, indent)}` : "";
			return `{ ${readonly}[${type.parameter} in ${print(type.parameterType, indent)}${as}]${optional}: ${print(type.templateType, indent)} }`;
		}
		case "namedTupleMember":
			return `${type.name}${type.isOptional ? "?" : ""}: ${print(type.element, indent)}`;
		case "optional":
			return `${print(type.elementType, indent, "operand")}?`;
		case "predicate":
			return `${type.asserts ? "asserts " : ""}${type.name}${type.targetType ? ` is ${print(type.targetType, indent)}` : ""}`;
		case "query":
			return `typeof ${print(type.queryType, indent)}`;
		case "reference": {
			const args = type.typeArguments?.length
				? `<${type.typeArguments.map((arg) => print(arg, indent)).join(", ")}>`
				: "";
			// A reference to a declaration an alias re-exports, such as with `@reexport`, uses the alias.
			const target = type.reflection;
			const name = target && !`.${type.name}`.endsWith(`.${target.name}`) ? target.name : type.name;
			return `${name}${args}`;
		}
		case "reflection": {
			const { declaration } = type;
			const [signature, ...others] = declaration.signatures ?? [];
			if (signature && others.length === 0 && !declaration.children && !declaration.indexSignatures) {
				const keyword = signature.kind === ReflectionKind.ConstructorSignature ? "new " : "";
				return wrap(`${keyword}${call(signature, " => ", indent)}`, [
					"union",
					"intersection",
					"operand",
					"check",
				]);
			}
			// One member per line, as a declaration's are, so a change to one is a one-line diff.
			const inner = `${indent}\t`;
			const entries = lines(declaration, inner);
			return entries.length ? `{\n${entries.join("\n")}\n${indent}}` : "{}";
		}
		case "rest":
			return `...${print(type.elementType, indent, "operand")}`;
		case "templateLiteral":
			return `\`${type.head}${type.tail.map(([part, text]) => `\${${print(part, indent)}}${text}`).join("")}\``;
		case "tuple":
			return `[${type.elements.map((element) => print(element, indent)).join(", ")}]`;
		case "typeOperator":
			return wrap(`${type.operator} ${print(type.target, indent, "operand")}`, ["operand"]);
	}
}

// Emit the declarations of a source package in `dir` into `out`, with a manifest whose exports
// name them, as the build's `dist/` would.
function declare(dir: string, out: string, manifest: Manifest) {
	const exports: Record<string, { types: string }> = {};
	let emit = false;
	for (const [subpath, target] of Object.entries(manifest.exports ?? {})) {
		const file = typeof target === "string" ? target : target.types;
		if (!file || file.endsWith(".css")) continue;
		if (file.endsWith(".d.ts")) {
			// Hand-written declarations, such as @moq/libav's, ship as they are.
			mkdirSync(dirname(join(out, file)), { recursive: true });
			cpSync(join(dir, file), join(out, file));
			exports[subpath] = { types: file };
		} else {
			exports[subpath] = { types: file.replace(/^\.\/src\//, "./").replace(/\.tsx?$/, ".d.ts") };
			emit = true;
		}
	}
	if (emit) {
		// Its exit status is ignored: a type error still emits, and the commit this renders was
		// checked when it was built.
		Bun.spawnSync(
			[
				"bunx",
				"tsc",
				"-p",
				"tsconfig.build.json",
				"--emitDeclarationOnly",
				"--declarationMap",
				"false",
				"--outDir",
				out,
			],
			{ cwd: dir, stdio: ["ignore", "inherit", "inherit"] },
		);
	}
	for (const { types } of Object.values(exports)) {
		if (!existsSync(join(out, types))) throw new Error(`${manifest.name} emitted no ${types}`);
	}
	writeFileSync(join(out, "package.json"), JSON.stringify({ ...manifest, exports }));
}

if (import.meta.main) {
	const [command, checkout, out] = process.argv.slice(2);
	if (command !== "baseline" || !checkout || !out) {
		console.error("usage: bun js/common/api.ts baseline <checkout> <out>");
		process.exit(2);
	}
	for (const entry of readdirSync(join(checkout, "js"), { withFileTypes: true })) {
		const dir = resolve(checkout, "js", entry.name);
		if (!entry.isDirectory() || !existsSync(join(dir, "package.json"))) continue;
		const manifest: Manifest = JSON.parse(readFileSync(join(dir, "package.json"), "utf8"));
		if (!manifest.scripts?.release) continue;
		const work = mkdtempSync(join(tmpdir(), "moq-api-baseline-"));
		try {
			declare(dir, join(work, "dist"), manifest);
			const found = await reports(dir, join(work, "dist"), false);
			mkdirSync(join(out, manifest.name), { recursive: true });
			for (const [file, report] of found) writeFileSync(join(out, manifest.name, file), report);
		} finally {
			rmSync(work, { recursive: true, force: true });
		}
	}
}
