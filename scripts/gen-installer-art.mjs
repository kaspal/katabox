#!/usr/bin/env node
// Branded NSIS installer artwork (ADR 0021, 0041), drawn from the app icon
// apps/desktop/src-tauri/icons/katabox.svg. Writes 24-bit BMPs (what NSIS needs) into
// apps/desktop/src-tauri/windows/, which are committed:
//   sidebar.bmp  164 × 314  welcome and finish pages: the icon's indigo gradient down the
//                           whole strip, with the cube (braces and chevron) near the top
//   header.bmp   150 × 57   the other pages: the full icon at the right of a white header
// No dependencies: scripts/lib/svg-raster.mjs rasterises the icon's rect, paths (curves
// flattened), gradient and transforms, with 4 × 4 supersampling.
//
// Usage: node scripts/gen-installer-art.mjs   (pnpm installer-art; from the repo root)
import fs from "node:fs";
import path from "node:path";
import { bmp, parseSvg, render } from "./lib/svg-raster.mjs";

const ROOT = process.cwd();
const SVG = path.join(ROOT, "apps/desktop/src-tauri/icons/katabox.svg");
const OUT = path.join(ROOT, "apps/desktop/src-tauri/windows");

const icon = parseSvg(fs.readFileSync(SVG, "utf8"));
// The first shape is the rounded tile; the rest is the cube and its marks.
const [tile, ...mark] = icon.shapes;
const cube = { viewBox: icon.viewBox, shapes: mark };

// The tile's gradient colours at its top and bottom (sampled, so they follow the SVG).
const top = tile.fill.color(128, 0);
const bottom = tile.fill.color(128, 256);

/** RGBA over an RGB background chosen per pixel. */
function composite(image, background) {
  const { width, height, px } = image;
  const out = new Uint8Array(width * height * 3);
  for (let y = 0; y < height; y++) {
    for (let x = 0; x < width; x++) {
      const i = y * width + x;
      const a = px[i * 4 + 3] / 255;
      const bg = background(x, y);
      for (let k = 0; k < 3; k++) out[i * 3 + k] = Math.round(bg[k] * (1 - a) + px[i * 4 + k] * a);
    }
  }
  return { width, height, px: out };
}

const lerp = (a, b, t) => a.map((c, k) => c + (b[k] - c) * t);
// The cube's own box in icon space (faces span x 50-206, y 40-218).
const cubeBox = { x0: 44, y0: 34, x1: 212, y1: 224 };

const outputs = {
  // The gradient approximated per row, from the tile's top colour to its bottom colour.
  "sidebar.bmp": composite(render(cube, 164, 314, { src: cubeBox, dst: { x0: 24, y0: 36, x1: 140, y1: 168 } }), (_, y) =>
    lerp(top, bottom, y / 313),
  ),
  "header.bmp": composite(render(icon, 150, 57, { dst: { x0: 99, y0: 6, x1: 144, y1: 51 } }), () => [255, 255, 255]),
};
fs.mkdirSync(OUT, { recursive: true });
for (const [name, image] of Object.entries(outputs)) {
  fs.writeFileSync(path.join(OUT, name), bmp(image));
  console.log(`wrote ${path.relative(ROOT, path.join(OUT, name))} (${image.width} × ${image.height})`);
}
