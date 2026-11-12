#!/usr/bin/env node
// Third-party notices for release builds (`pnpm notices`, ADR 0021). Writes
// apps/desktop/src-tauri/resources/THIRD_PARTY_NOTICES.txt, which the installers bundle
// and Settings → About shows. Three parts:
//   1. Rust crates linked into the app: `cargo metadata` for the target platform,
//      following normal dependencies of katabox-desktop (not build or dev ones), with
//      each crate's own license and notice files;
//   2. npm packages in the UI bundle (`pnpm licenses list --prod`), with their files;
//   3. docs/third-party-notices.md (language servers, runtimes, linking notes).
// Nothing here downloads anything: crate sources are already in the cargo registry
// after a build, npm packages in node_modules.
//
// Usage: node scripts/gen-notices.mjs [--target x86_64-pc-windows-msvc] (from the repo root)
import { execFileSync, execSync } from "node:child_process";
import fs from "node:fs";
import path from "node:path";

const ROOT = process.cwd();
const OUT = path.join(ROOT, "apps/desktop/src-tauri/resources/THIRD_PARTY_NOTICES.txt");
const args = process.argv.slice(2);
const targetArg = args.indexOf("--target");
const target =
  targetArg >= 0
    ? args[targetArg + 1]
    : execFileSync("rustc", ["-vV"], { encoding: "utf8" }).match(/^host: (.+)$/m)[1].trim();

// Standard texts for packages that ship no license file of their own (Appendix).
const TEXTS_DIR = path.join(ROOT, "scripts/license-texts");
const STANDARD = new Set(fs.readdirSync(TEXTS_DIR).map((n) => n.replace(/\.txt$/, "")));
const appendix = new Set();
/** SPDX ids in an expression such as "MIT OR Apache-2.0" or "MIT/Apache-2.0". */
const spdxIds = (expr) =>
  (expr ?? "")
    .split(/\s+(?:OR|AND|WITH)\s+|[/()]/)
    .map((s) => s.trim())
    .filter(Boolean);
/** The note for a package without a license file; throws for a license with no standard text. */
function standardNote(name, expr, holders) {
  const ids = spdxIds(expr);
  const unknown = ids.filter((id) => !STANDARD.has(id));
  if (ids.length === 0 || unknown.length) {
    throw new Error(`${name}: no license file and no standard text for ${unknown.join(", ") || "its license"}`);
  }
  ids.forEach((id) => appendix.add(id));
  return `(The package ships no license file. Copyright holders: ${holders || "its authors"}. ` +
    `License: ${expr}; the standard text is in the Appendix.)`;
}

const LICENSE_FILE = /^(licen[cs]e|copying|notice|copyright|authors)([-._].*)?$/i;
const RULE = "=".repeat(78);
const rule = (title) => `\n${RULE}\n${title}\n${RULE}\n`;

/** License-like files at the top of a package folder, sorted, de-duplicated by text. */
function licenseTexts(dir) {
  let names = [];
  try {
    names = fs.readdirSync(dir).filter((n) => LICENSE_FILE.test(n) && fs.statSync(path.join(dir, n)).isFile());
  } catch {
    return [];
  }
  const seen = new Set();
  const out = [];
  for (const n of names.sort()) {
    const text = fs.readFileSync(path.join(dir, n), "utf8").replace(/\r\n/g, "\n").trim();
    if (!text || seen.has(text)) continue;
    seen.add(text);
    out.push({ file: n, text });
  }
  return out;
}

// --- 1. Rust ------------------------------------------------------------------------
const meta = JSON.parse(
  execFileSync(
    "cargo",
    ["metadata", "--format-version", "1", "--locked", "--filter-platform", target],
    { encoding: "utf8", maxBuffer: 256 * 1024 * 1024 },
  ),
);
const pkgs = new Map(meta.packages.map((p) => [p.id, p]));
const nodes = new Map(meta.resolve.nodes.map((n) => [n.id, n]));
const members = new Set(meta.workspace_members);
const app = meta.packages.find((p) => p.name === "katabox-desktop");
if (!app) throw new Error("katabox-desktop is not in the workspace");
const shipped = new Set();
const stack = [app.id];
while (stack.length) {
  const id = stack.pop();
  if (shipped.has(id)) continue;
  shipped.add(id);
  for (const dep of nodes.get(id)?.deps ?? []) {
    // A normal dependency has `kind: null`; build scripts and dev-dependencies are not shipped.
    if (dep.dep_kinds.some((k) => k.kind === null)) stack.push(dep.pkg);
  }
}
const crates = [...shipped]
  .filter((id) => !members.has(id))
  .map((id) => pkgs.get(id))
  .sort((a, b) => a.name.localeCompare(b.name) || a.version.localeCompare(b.version));

// --- 2. npm -------------------------------------------------------------------------
const pnpm = "pnpm";
const npmJson = JSON.parse(
  execSync(`${pnpm} licenses list --json --prod`, {
    cwd: path.join(ROOT, "apps/desktop"),
    encoding: "utf8",
    maxBuffer: 64 * 1024 * 1024,
  }),
);
const npm = Object.entries(npmJson)
  .flatMap(([license, list]) => list.map((p) => ({ ...p, license })))
  .sort((a, b) => a.name.localeCompare(b.name));

// --- Write --------------------------------------------------------------------------
const parts = [];
parts.push(`KataBox third-party notices
${"-".repeat(27)}
KataBox is free software under MIT OR Apache-2.0. It includes the components below,
each under its own license. This file is generated by scripts/gen-notices.mjs for
${target}.

Contents: Rust crates (${crates.length}), npm packages (${npm.length}), language servers
and runtimes.`);

parts.push(rule("Part 1. Rust crates linked into KataBox"));
let missing = 0;
for (const c of crates) {
  const texts = licenseTexts(path.dirname(c.manifest_path));
  parts.push(`\n--- ${c.name} ${c.version} (${c.license ?? c.license_file ?? "unknown license"})`);
  if (c.repository) parts.push(c.repository);
  if (texts.length === 0) {
    missing++;
    parts.push(standardNote(c.name, c.license, (c.authors ?? []).join("; ")));
  }
  for (const t of texts) parts.push(`\n[${t.file}]\n${t.text}`);
}

parts.push(rule("Part 2. npm packages in the user interface"));
for (const p of npm) {
  const dir = p.paths?.[0];
  const texts = dir ? licenseTexts(dir) : [];
  parts.push(`\n--- ${p.name} ${p.versions.join(", ")} (${p.license})`);
  if (p.homepage) parts.push(p.homepage);
  if (texts.length === 0) {
    missing++;
    parts.push(standardNote(p.name, p.license, p.author));
  }
  for (const t of texts) parts.push(`\n[${t.file}]\n${t.text}`);
}

parts.push(rule("Part 3. Language servers, runtimes and linking notes"));
parts.push(fs.readFileSync(path.join(ROOT, "docs/third-party-notices.md"), "utf8").replace(/\r\n/g, "\n").trim());

if (appendix.size) {
  parts.push(rule("Appendix. Standard license texts"));
  for (const id of [...appendix].sort()) {
    parts.push(`
--- ${id}

${fs.readFileSync(path.join(TEXTS_DIR, `${id}.txt`), "utf8").trim()}`);
  }
}

fs.mkdirSync(path.dirname(OUT), { recursive: true });
fs.writeFileSync(OUT, parts.join("\n") + "\n");
const kb = Math.round(fs.statSync(OUT).size / 1024);
console.log(
  `wrote ${path.relative(ROOT, OUT)}: ${crates.length} crates, ${npm.length} npm packages, ${kb} KB` +
    (missing ? ` (${missing} without a license file)` : ""),
);
