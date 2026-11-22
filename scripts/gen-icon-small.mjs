#!/usr/bin/env node
// The small app-icon sizes (ADR 0041). `pnpm tauri icon icons/katabox.svg` makes every
// size from the master; at 16, 24 and 32 px its braces turn to mush, so this script
// redraws those layers of icon.ico (and 32x32.png) from icons/katabox-small.svg (the
// cube and a bolder chevron), keeps the ICO's larger layers, and reads the result back.
// No dependencies: scripts/lib/svg-raster.mjs draws the SVG and writes PNG and ICO.
//
// Usage (from the repo root, after `pnpm tauri icon icons/katabox.svg`):
//   node scripts/gen-icon-small.mjs            (pnpm icon-small)
//   node scripts/gen-icon-small.mjs --preview DIR   also writes the 16/24/32 PNGs to DIR
import fs from "node:fs";
import path from "node:path";
import { parseSvg, png, readIco, render, writeIco } from "./lib/svg-raster.mjs";

const ROOT = process.cwd();
const ICONS = path.join(ROOT, "apps/desktop/src-tauri/icons");
const SMALL = [16, 24, 32];

const small = parseSvg(fs.readFileSync(path.join(ICONS, "katabox-small.svg"), "utf8"));
const layers = Object.fromEntries(SMALL.map((n) => [n, png(render(small, n, n, { S: 8 }))]));

const icoPath = path.join(ICONS, "icon.ico");
const kept = readIco(fs.readFileSync(icoPath)).filter((e) => !SMALL.includes(e.width));
const entries = [
  ...SMALL.map((n) => ({ width: n, height: n, bitCount: 32, data: layers[n] })),
  ...kept,
];
fs.writeFileSync(icoPath, writeIco(entries));
fs.writeFileSync(path.join(ICONS, "32x32.png"), layers[32]);

const preview = process.argv.indexOf("--preview");
if (preview !== -1) {
  const dir = path.resolve(process.argv[preview + 1]);
  fs.mkdirSync(dir, { recursive: true });
  for (const n of SMALL) fs.writeFileSync(path.join(dir, `small-${n}.png`), layers[n]);
}

// Read back what was written.
const back = readIco(fs.readFileSync(icoPath));
console.log(
  `icon.ico: ${back.length} layers: ${back
    .map((e) => `${e.width}x${e.height} ${e.data.subarray(1, 4).toString() === "PNG" ? "png" : "bmp"} ${e.data.length} B`)
    .join(", ")}`,
);
for (const n of SMALL) {
  if (!back.some((e) => e.width === n && e.data.equals(layers[n]))) throw new Error(`layer ${n} missing after write`);
}
console.log("32x32.png: redrawn from katabox-small.svg");
