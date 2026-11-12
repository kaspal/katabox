#!/usr/bin/env node
// Branded NSIS installer artwork (ADR 0021), drawn from the app icon
// apps/desktop/src-tauri/icons/katabox.svg. Writes 24-bit BMPs (what NSIS needs) into
// apps/desktop/src-tauri/windows/, which are committed:
//   sidebar.bmp  164 × 314  welcome and finish pages (installer and uninstaller)
//   header.bmp   150 × 57   the header of the other pages (installer and uninstaller)
// No dependencies: the icon is a rounded rectangle plus straight-edged paths, so this
// script rasterizes those shapes itself, with 4 × 4 supersampling.
//
// Usage: node scripts/gen-installer-art.mjs   (pnpm installer-art; from the repo root)
import fs from "node:fs";
import path from "node:path";

const ROOT = process.cwd();
const SVG = path.join(ROOT, "apps/desktop/src-tauri/icons/katabox.svg");
const OUT = path.join(ROOT, "apps/desktop/src-tauri/windows");

// --- Parse the icon ------------------------------------------------------------------
const svg = fs.readFileSync(SVG, "utf8").replace(/<!--[\s\S]*?-->/g, "");
const attr = (tag, name) => tag.match(new RegExp(`\\b${name}="([^"]*)"`))?.[1];
const rectTag = svg.match(/<rect\b[^>]*>/)[0];
const rect = {
  x: +attr(rectTag, "x"),
  y: +attr(rectTag, "y"),
  w: +attr(rectTag, "width"),
  h: +attr(rectTag, "height"),
  r: +attr(rectTag, "rx"),
  fill: attr(rectTag, "fill"),
};
const groupTag = svg.match(/<g\b[^>]*>/)[0];
const stroke = { color: attr(groupTag, "stroke"), width: +attr(groupTag, "stroke-width") };
const faces = [...svg.matchAll(/<path\b[^>]*>/g)].map(([tag]) => ({
  fill: attr(tag, "fill"),
  // "M x y L x y ... Z": straight segments only.
  points: attr(tag, "d")
    .replace(/[MLZ]/g, " ")
    .trim()
    .split(/\s+/)
    .map(Number)
    .reduce((acc, v, i, all) => (i % 2 ? acc : [...acc, [v, all[i + 1]]]), []),
}));
// The cube's bounding box (the faces), to place it on its own.
const xs = faces.flatMap((f) => f.points.map((p) => p[0]));
const ys = faces.flatMap((f) => f.points.map((p) => p[1]));
const cube = { x0: Math.min(...xs), y0: Math.min(...ys), x1: Math.max(...xs), y1: Math.max(...ys) };

const rgb = (hex) => [1, 3, 5].map((i) => parseInt(hex.slice(i, i + 2), 16));

// --- Geometry ----------------------------------------------------------------------
function inPolygon(px, py, pts) {
  let inside = false;
  for (let i = 0, j = pts.length - 1; i < pts.length; j = i++) {
    const [xi, yi] = pts[i];
    const [xj, yj] = pts[j];
    if (yi > py !== yj > py && px < ((xj - xi) * (py - yi)) / (yj - yi) + xi) inside = !inside;
  }
  return inside;
}

function distToSegment(px, py, [ax, ay], [bx, by]) {
  const dx = bx - ax;
  const dy = by - ay;
  const t = Math.max(0, Math.min(1, ((px - ax) * dx + (py - ay) * dy) / (dx * dx + dy * dy)));
  return Math.hypot(px - (ax + t * dx), py - (ay + t * dy));
}

function inRoundedRect(px, py, { x, y, w, h, r }) {
  if (px < x || py < y || px > x + w || py > y + h) return false;
  const cx = Math.max(x + r, Math.min(px, x + w - r));
  const cy = Math.max(y + r, Math.min(py, y + h - r));
  return Math.hypot(px - cx, py - cy) <= r;
}

/** The icon's color at icon coordinates (x, y), or null where it is transparent. */
function iconColor(x, y, withBackground) {
  let color = withBackground && inRoundedRect(x, y, rect) ? rgb(rect.fill) : null;
  for (const f of faces) {
    if (inPolygon(x, y, f.points)) color = rgb(f.fill);
    // The stroke (round joins) is drawn over the face, like SVG paints it.
    const pts = f.points;
    for (let i = 0; i < pts.length; i++) {
      if (distToSegment(x, y, pts[i], pts[(i + 1) % pts.length]) <= stroke.width / 2) {
        color = rgb(stroke.color);
        break;
      }
    }
  }
  return color;
}

// --- Images ------------------------------------------------------------------------
/**
 * Draw a width × height image: `background`, then the icon mapped from the icon-space
 * box `src` ({x0, y0, x1, y1}) into the image-space box `dst`, 4 × 4 samples per pixel.
 */
function render(width, height, background, src, dst, withBackground) {
  const px = new Uint8Array(width * height * 3);
  const scale = Math.min((dst.x1 - dst.x0) / (src.x1 - src.x0), (dst.y1 - dst.y0) / (src.y1 - src.y0));
  const ox = dst.x0 + ((dst.x1 - dst.x0) - (src.x1 - src.x0) * scale) / 2;
  const oy = dst.y0 + ((dst.y1 - dst.y0) - (src.y1 - src.y0) * scale) / 2;
  const S = 4;
  for (let y = 0; y < height; y++) {
    for (let x = 0; x < width; x++) {
      const sum = [0, 0, 0];
      for (let sy = 0; sy < S; sy++) {
        for (let sx = 0; sx < S; sx++) {
          const ix = src.x0 + (x + (sx + 0.5) / S - ox) / scale;
          const iy = src.y0 + (y + (sy + 0.5) / S - oy) / scale;
          const c = iconColor(ix, iy, withBackground) ?? background;
          for (let k = 0; k < 3; k++) sum[k] += c[k];
        }
      }
      for (let k = 0; k < 3; k++) px[(y * width + x) * 3 + k] = Math.round(sum[k] / (S * S));
    }
  }
  return { width, height, px };
}

/** A 24-bit, bottom-up Windows BMP. */
function bmp({ width, height, px }) {
  const row = Math.ceil((width * 3) / 4) * 4;
  const size = 54 + row * height;
  const b = Buffer.alloc(size);
  b.write("BM", 0);
  b.writeUInt32LE(size, 2);
  b.writeUInt32LE(54, 10);
  b.writeUInt32LE(40, 14);
  b.writeInt32LE(width, 18);
  b.writeInt32LE(height, 22);
  b.writeUInt16LE(1, 26);
  b.writeUInt16LE(24, 28);
  b.writeUInt32LE(row * height, 34);
  b.writeInt32LE(2835, 38); // 72 dpi
  b.writeInt32LE(2835, 42);
  for (let y = 0; y < height; y++) {
    const at = 54 + (height - 1 - y) * row;
    for (let x = 0; x < width; x++) {
      const i = (y * width + x) * 3;
      b[at + x * 3] = px[i + 2]; // B
      b[at + x * 3 + 1] = px[i + 1]; // G
      b[at + x * 3 + 2] = px[i]; // R
    }
  }
  return b;
}

const brand = rgb(rect.fill);
const white = [255, 255, 255];
const outputs = {
  // The cube on the brand colour, in the upper part of the tall welcome-page strip.
  "sidebar.bmp": render(164, 314, brand, cube, { x0: 22, y0: 40, x1: 142, y1: 180 }, false),
  // The full icon (rounded square and cube) at the right of a white header.
  "header.bmp": render(
    150,
    57,
    white,
    { x0: rect.x, y0: rect.y, x1: rect.x + rect.w, y1: rect.y + rect.h },
    { x0: 99, y0: 6, x1: 144, y1: 51 },
    true,
  ),
};
fs.mkdirSync(OUT, { recursive: true });
for (const [name, image] of Object.entries(outputs)) {
  fs.writeFileSync(path.join(OUT, name), bmp(image));
  console.log(`wrote ${path.relative(ROOT, path.join(OUT, name))} (${image.width} × ${image.height})`);
}
