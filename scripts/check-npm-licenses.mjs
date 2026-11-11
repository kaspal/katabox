#!/usr/bin/env node
// npm license check (`pnpm licenses:check`): every package in the pnpm workspace,
// dependencies and devDependencies alike, must use a license from ALLOW, or be listed in
// EXCEPTIONS with a reason. Mirrors deny.toml for the Rust side.
//
// Usage: node scripts/check-npm-licenses.mjs   (run from the repo root; pnpm on PATH)
import { execFileSync } from "node:child_process";

const ALLOW = new Set([
  // Same as deny.toml.
  "MIT",
  "MIT-0",
  "Apache-2.0",
  "Apache-2.0 WITH LLVM-exception",
  "BSD-2-Clause",
  "BSD-3-Clause",
  "Zlib",
  "0BSD",
  "CC0-1.0",
  "Unlicense",
  "Unicode-3.0",
  "MPL-2.0", // weak, file-level copyleft (lightningcss via Tailwind, build time)
  // npm only.
  "ISC", // permissive, MIT-equivalent
  "OFL-1.1", // bundled fonts: Inter, JetBrains Mono, KaTeX (ADR 0012)
  "BlueOak-1.0.0", // permissive (dev tooling)
]);

// Package name → reason, for anything outside ALLOW that the maintainer approved.
const EXCEPTIONS = new Map([]);

/** Is an SPDX expression acceptable? `OR`: one allowed branch is enough; `AND`: all. */
function allowed(expr) {
  const clean = expr.replace(/[()]/g, "").trim();
  if (ALLOW.has(clean)) return true;
  // "A OR B": one allowed alternative is enough. "A AND B": all must be allowed.
  if (/ OR /.test(clean)) return clean.split(/ OR /).some((p) => allowed(p));
  if (/ AND /.test(clean)) return clean.split(/ AND /).every((p) => allowed(p));
  // Legacy "MIT/Apache-2.0" style.
  if (clean.includes("/")) return clean.split("/").some((p) => allowed(p));
  return false;
}

const pnpm = process.platform === "win32" ? "pnpm.cmd" : "pnpm";
const json = execFileSync(pnpm, ["licenses", "list", "--json", "-r"], {
  encoding: "utf8",
  shell: process.platform === "win32",
  maxBuffer: 64 * 1024 * 1024,
});
const byLicense = JSON.parse(json);

let bad = 0;
let total = 0;
for (const [license, pkgs] of Object.entries(byLicense)) {
  for (const pkg of pkgs) {
    total++;
    if (allowed(license)) continue;
    const why = EXCEPTIONS.get(pkg.name);
    if (why) {
      console.log(`exception  ${pkg.name} (${license}): ${why}`);
      continue;
    }
    bad++;
    console.log(`DENIED     ${pkg.name}@${pkg.versions?.join(", ") ?? "?"}: ${license}`);
  }
}
console.log(`\n${total} package(s) checked, ${bad} with a license outside the allow-list`);
process.exit(bad === 0 ? 0 : 1);
