// A tiny SVG rasteriser for the app icon (ADR 0021, 0041), with no dependencies. It
// understands exactly what icons/katabox*.svg use:
//   <rect> (x, y, width, height, rx), <path> (M L H V C S Q Z, absolute or relative),
//   <g> nesting, transform="matrix(...)" / "translate(...)", fill (a colour, "none" or a
//   url(#id) to a <linearGradient> in objectBoundingBox units), stroke, stroke-width,
//   stroke-opacity, fill-opacity, opacity. Curves are flattened to segments; strokes are
//   drawn as the points within half the width of the outline (round caps and joins).
// Rendering supersamples each pixel (S × S) and returns RGBA bytes.
import zlib from "node:zlib";

// --- Parsing --------------------------------------------------------------------------

const attrsOf = (src) => Object.fromEntries([...src.matchAll(/([\w:-]+)="([^"]*)"/g)].map((m) => [m[1], m[2]]));

function parseTransform(s) {
  let m = [1, 0, 0, 1, 0, 0];
  if (!s) return m;
  for (const [, fn, args] of s.matchAll(/(matrix|translate|scale)\s*\(([^)]*)\)/g)) {
    const n = args.split(/[\s,]+/).filter(Boolean).map(Number);
    const t =
      fn === "matrix"
        ? n
        : fn === "translate"
          ? [1, 0, 0, 1, n[0], n[1] ?? 0]
          : [n[0], 0, 0, n[1] ?? n[0], 0, 0];
    m = mul(m, t);
  }
  return m;
}

/** a ∘ b: apply b, then a. Matrices are [a b c d e f] as in SVG. */
function mul(a, b) {
  return [
    a[0] * b[0] + a[2] * b[1],
    a[1] * b[0] + a[3] * b[1],
    a[0] * b[2] + a[2] * b[3],
    a[1] * b[2] + a[3] * b[3],
    a[0] * b[4] + a[2] * b[5] + a[4],
    a[1] * b[4] + a[3] * b[5] + a[5],
  ];
}

const apply = (m, [x, y]) => [m[0] * x + m[2] * y + m[4], m[1] * x + m[3] * y + m[5]];

export function hex(c) {
  const h = c.replace("#", "");
  const full = h.length === 3 ? [...h].map((x) => x + x).join("") : h;
  return [0, 2, 4].map((i) => parseInt(full.slice(i, i + 2), 16));
}

/** Path data → subpaths of points (curves flattened), and whether each is closed. */
export function flattenPath(d) {
  const tokens = [...d.matchAll(/[MLHVCSQZmlhvcsqz]|-?(?:\d+\.?\d*|\.\d+)(?:e-?\d+)?/g)].map((m) => m[0]);
  const subs = [];
  let cur = null;
  let x = 0;
  let y = 0;
  let sx = 0;
  let sy = 0;
  let lastCtrl = null;
  let cmd = "";
  let i = 0;
  const num = () => Number(tokens[i++]);
  const STEPS = 24;
  const cubic = (x1, y1, x2, y2, x3, y3) => {
    for (let k = 1; k <= STEPS; k++) {
      const t = k / STEPS;
      const u = 1 - t;
      cur.pts.push([
        u * u * u * x + 3 * u * u * t * x1 + 3 * u * t * t * x2 + t * t * t * x3,
        u * u * u * y + 3 * u * u * t * y1 + 3 * u * t * t * y2 + t * t * t * y3,
      ]);
    }
    lastCtrl = [x2, y2];
    x = x3;
    y = y3;
  };
  while (i < tokens.length) {
    if (/[A-Za-z]/.test(tokens[i])) cmd = tokens[i++];
    const rel = cmd === cmd.toLowerCase();
    const C = cmd.toUpperCase();
    const ox = rel ? x : 0;
    const oy = rel ? y : 0;
    if (C === "Z") {
      if (cur) cur.closed = true;
      x = sx;
      y = sy;
      lastCtrl = null;
      continue;
    }
    if (C === "M") {
      x = ox + num();
      y = oy + num();
      sx = x;
      sy = y;
      cur = { pts: [[x, y]], closed: false };
      subs.push(cur);
      cmd = rel ? "l" : "L"; // following pairs are line-tos
      lastCtrl = null;
    } else if (C === "L") {
      x = ox + num();
      y = oy + num();
      cur.pts.push([x, y]);
      lastCtrl = null;
    } else if (C === "H") {
      x = (rel ? x : 0) + num();
      cur.pts.push([x, y]);
      lastCtrl = null;
    } else if (C === "V") {
      y = (rel ? y : 0) + num();
      cur.pts.push([x, y]);
      lastCtrl = null;
    } else if (C === "C") {
      const a = [ox + num(), oy + num(), ox + num(), oy + num(), ox + num(), oy + num()];
      cubic(...a);
    } else if (C === "S") {
      const [rx, ry] = lastCtrl ? [2 * x - lastCtrl[0], 2 * y - lastCtrl[1]] : [x, y];
      const a = [ox + num(), oy + num(), ox + num(), oy + num()];
      cubic(rx, ry, ...a);
    } else if (C === "Q") {
      const qx = ox + num();
      const qy = oy + num();
      const ex = ox + num();
      const ey = oy + num();
      cubic(x + (2 / 3) * (qx - x), y + (2 / 3) * (qy - y), ex + (2 / 3) * (qx - ex), ey + (2 / 3) * (qy - ey), ex, ey);
    } else {
      throw new Error(`svg-raster: unsupported path command ${cmd}`);
    }
  }
  return subs;
}

/**
 * Parse an icon SVG into a list of shapes in viewBox coordinates:
 * { polys: [[x, y]...][], closed: bool[], fill, stroke, strokeWidth, bbox }.
 */
export function parseSvg(svg) {
  const src = svg.replace(/<!--[\s\S]*?-->/g, "");
  const viewBox = (attrsOf(src.match(/<svg\b[^>]*>/)[0]).viewBox ?? "0 0 256 256").split(/[\s,]+/).map(Number);
  const gradients = {};
  for (const m of src.matchAll(/<linearGradient\b([^>]*)>([\s\S]*?)<\/linearGradient>/g)) {
    const a = attrsOf(m[1]);
    gradients[a.id] = {
      x1: +(a.x1 ?? 0),
      y1: +(a.y1 ?? 0),
      x2: +(a.x2 ?? 1),
      y2: +(a.y2 ?? 0),
      stops: [...m[2].matchAll(/<stop\b([^>]*)>/g)].map(([, s]) => {
        const sa = attrsOf(s);
        return { at: parseFloat(sa.offset), color: hex(sa["stop-color"]) };
      }),
    };
  }
  const body = src.replace(/<defs>[\s\S]*?<\/defs>/g, "");
  const shapes = [];
  const stack = [{ m: [1, 0, 0, 1, 0, 0], style: {} }];
  const STYLE = ["fill", "stroke", "stroke-width", "stroke-opacity", "fill-opacity", "opacity"];
  for (const [, close, tag, rest] of body.matchAll(/<(\/?)(\w+)\b([^>]*?)\/?>/g)) {
    if (tag === "svg") continue;
    if (close) {
      if (tag === "g") stack.pop();
      continue;
    }
    const a = attrsOf(rest);
    const top = stack[stack.length - 1];
    const m = mul(top.m, parseTransform(a.transform));
    const style = { ...top.style };
    for (const k of STYLE) if (a[k] !== undefined) style[k] = a[k];
    if (k_opacity(top.style) !== 1 && a.opacity !== undefined) style.opacity = String(k_opacity(top.style) * +a.opacity);
    if (tag === "g") {
      stack.push({ m, style });
      continue;
    }
    let subs;
    if (tag === "rect") {
      subs = [{ pts: roundedRect(+(a.x ?? 0), +(a.y ?? 0), +a.width, +a.height, +(a.rx ?? 0)), closed: true }];
    } else if (tag === "path") {
      subs = flattenPath(a.d);
    } else {
      continue;
    }
    const local = subs.flatMap((s) => s.pts);
    const xs = local.map((p) => p[0]);
    const ys = local.map((p) => p[1]);
    const bbox = { x: Math.min(...xs), y: Math.min(...ys), w: Math.max(...xs) - Math.min(...xs), h: Math.max(...ys) - Math.min(...ys) };
    const scale = Math.sqrt(Math.abs(m[0] * m[3] - m[1] * m[2]));
    const fill = paint(style.fill ?? "#000000", gradients, bbox, m);
    const stroke = style.stroke && style.stroke !== "none" ? paint(style.stroke, gradients, bbox, m) : null;
    const opacity = k_opacity(style);
    shapes.push({
      polys: subs.map((s) => s.pts.map((p) => apply(m, p))),
      closed: subs.map((s) => s.closed),
      fill: fill && { ...fill, alpha: opacity * +(style["fill-opacity"] ?? 1) },
      stroke: stroke && { ...stroke, alpha: opacity * +(style["stroke-opacity"] ?? 1) },
      strokeWidth: +(style["stroke-width"] ?? 1) * scale,
    });
  }
  return { viewBox, shapes };
}

const k_opacity = (style) => +(style.opacity ?? 1);

function roundedRect(x, y, w, h, r) {
  r = Math.min(r, w / 2, h / 2);
  if (r <= 0) return [[x, y], [x + w, y], [x + w, y + h], [x, y + h]];
  const pts = [];
  const arc = (cx, cy, from) => {
    for (let k = 0; k <= 12; k++) {
      const a = from + (k / 12) * (Math.PI / 2);
      pts.push([cx + r * Math.cos(a), cy + r * Math.sin(a)]);
    }
  };
  arc(x + w - r, y + r, -Math.PI / 2);
  arc(x + w - r, y + h - r, 0);
  arc(x + r, y + h - r, Math.PI / 2);
  arc(x + r, y + r, Math.PI);
  return pts;
}

/** A paint: a solid colour, or a gradient as a function of global (x, y). */
function paint(value, gradients, bbox, m) {
  if (value === "none") return null;
  const id = value.match(/url\(#([^)]+)\)/)?.[1];
  if (!id) return { color: () => hex(value) };
  const g = gradients[id];
  // objectBoundingBox: gradient points in the shape's local bbox, then the transform.
  const p1 = apply(m, [bbox.x + g.x1 * bbox.w, bbox.y + g.y1 * bbox.h]);
  const p2 = apply(m, [bbox.x + g.x2 * bbox.w, bbox.y + g.y2 * bbox.h]);
  const dx = p2[0] - p1[0];
  const dy = p2[1] - p1[1];
  const len2 = dx * dx + dy * dy || 1;
  return {
    color: (x, y) => {
      const t = Math.max(0, Math.min(1, ((x - p1[0]) * dx + (y - p1[1]) * dy) / len2));
      const stops = g.stops;
      if (t <= stops[0].at) return stops[0].color;
      for (let k = 1; k < stops.length; k++) {
        if (t <= stops[k].at) {
          const u = (t - stops[k - 1].at) / (stops[k].at - stops[k - 1].at || 1);
          return stops[k - 1].color.map((c, j) => c + (stops[k].color[j] - c) * u);
        }
      }
      return stops[stops.length - 1].color;
    },
  };
}

// --- Geometry -------------------------------------------------------------------------

function insideEvenOdd(px, py, polys) {
  let inside = false;
  for (const pts of polys) {
    for (let i = 0, j = pts.length - 1; i < pts.length; j = i++) {
      const [xi, yi] = pts[i];
      const [xj, yj] = pts[j];
      if (yi > py !== yj > py && px < ((xj - xi) * (py - yi)) / (yj - yi) + xi) inside = !inside;
    }
  }
  return inside;
}

function distToSegment(px, py, [ax, ay], [bx, by]) {
  const dx = bx - ax;
  const dy = by - ay;
  const l2 = dx * dx + dy * dy;
  const t = l2 === 0 ? 0 : Math.max(0, Math.min(1, ((px - ax) * dx + (py - ay) * dy) / l2));
  return Math.hypot(px - (ax + t * dx), py - (ay + t * dy));
}

function nearOutline(px, py, shape) {
  const half = shape.strokeWidth / 2;
  for (let s = 0; s < shape.polys.length; s++) {
    const pts = shape.polys[s];
    const n = shape.closed[s] ? pts.length : pts.length - 1;
    for (let i = 0; i < n; i++) {
      if (distToSegment(px, py, pts[i], pts[(i + 1) % pts.length]) <= half) return true;
    }
  }
  return false;
}

const over = (dst, src, a) => [0, 1, 2].map((k) => dst[k] * (1 - a) + src[k] * a);

/** The colour and coverage at viewBox point (x, y): [r, g, b, a] (straight alpha). */
export function sample(shapes, x, y) {
  let rgb = [0, 0, 0];
  let alpha = 0;
  const layer = (color, a) => {
    // Straight-alpha "over".
    const out = a + alpha * (1 - a);
    if (out > 0) rgb = rgb.map((c, k) => (color[k] * a + c * alpha * (1 - a)) / out);
    alpha = out;
  };
  for (const s of shapes) {
    if (s.fill && insideEvenOdd(x, y, s.polys)) layer(s.fill.color(x, y), s.fill.alpha);
    if (s.stroke && nearOutline(x, y, s)) layer(s.stroke.color(x, y), s.stroke.alpha);
  }
  return [...rgb, alpha];
}

/**
 * Render the viewBox box `src` ({x0, y0, x1, y1}, default the whole viewBox) into a
 * width × height RGBA image, fitted and centred in `dst` (default the whole image).
 */
export function render(parsed, width, height, { src, dst, S = 4 } = {}) {
  const [vx, vy, vw, vh] = parsed.viewBox;
  const s0 = src ?? { x0: vx, y0: vy, x1: vx + vw, y1: vy + vh };
  const d0 = dst ?? { x0: 0, y0: 0, x1: width, y1: height };
  const scale = Math.min((d0.x1 - d0.x0) / (s0.x1 - s0.x0), (d0.y1 - d0.y0) / (s0.y1 - s0.y0));
  const ox = d0.x0 + (d0.x1 - d0.x0 - (s0.x1 - s0.x0) * scale) / 2;
  const oy = d0.y0 + (d0.y1 - d0.y0 - (s0.y1 - s0.y0) * scale) / 2;
  const px = new Uint8Array(width * height * 4);
  for (let y = 0; y < height; y++) {
    for (let x = 0; x < width; x++) {
      const acc = [0, 0, 0, 0];
      for (let sy = 0; sy < S; sy++) {
        for (let sx = 0; sx < S; sx++) {
          const ix = s0.x0 + (x + (sx + 0.5) / S - ox) / scale;
          const iy = s0.y0 + (y + (sy + 0.5) / S - oy) / scale;
          const [r, g, b, a] = sample(parsed.shapes, ix, iy);
          // Premultiplied accumulation.
          acc[0] += r * a;
          acc[1] += g * a;
          acc[2] += b * a;
          acc[3] += a;
        }
      }
      const i = (y * width + x) * 4;
      const a = acc[3] / (S * S);
      for (let k = 0; k < 3; k++) px[i + k] = acc[3] > 0 ? Math.round(acc[k] / acc[3]) : 0;
      px[i + 3] = Math.round(a * 255);
    }
  }
  return { width, height, px };
}

/** Flatten an RGBA image onto a solid background (for BMPs). */
export function flatten({ width, height, px }, background) {
  const out = new Uint8Array(width * height * 3);
  for (let i = 0; i < width * height; i++) {
    const a = px[i * 4 + 3] / 255;
    for (let k = 0; k < 3; k++) out[i * 3 + k] = Math.round(background[k] * (1 - a) + px[i * 4 + k] * a);
  }
  return { width, height, px: out };
}

// --- Encoders -------------------------------------------------------------------------

const CRC = (() => {
  const t = new Uint32Array(256);
  for (let n = 0; n < 256; n++) {
    let c = n;
    for (let k = 0; k < 8; k++) c = c & 1 ? 0xedb88320 ^ (c >>> 1) : c >>> 1;
    t[n] = c >>> 0;
  }
  return t;
})();

function crc32(buf) {
  let c = 0xffffffff;
  for (const b of buf) c = CRC[(c ^ b) & 0xff] ^ (c >>> 8);
  return (c ^ 0xffffffff) >>> 0;
}

function chunk(type, data) {
  const len = Buffer.alloc(4);
  len.writeUInt32BE(data.length);
  const td = Buffer.concat([Buffer.from(type, "ascii"), data]);
  const crc = Buffer.alloc(4);
  crc.writeUInt32BE(crc32(td));
  return Buffer.concat([len, td, crc]);
}

/** An 8-bit RGBA PNG. */
export function png({ width, height, px }) {
  const ihdr = Buffer.alloc(13);
  ihdr.writeUInt32BE(width, 0);
  ihdr.writeUInt32BE(height, 4);
  ihdr[8] = 8; // bit depth
  ihdr[9] = 6; // RGBA
  const raw = Buffer.alloc((width * 4 + 1) * height);
  for (let y = 0; y < height; y++) {
    raw[y * (width * 4 + 1)] = 0; // filter: none
    Buffer.from(px.buffer, px.byteOffset + y * width * 4, width * 4).copy(raw, y * (width * 4 + 1) + 1);
  }
  return Buffer.concat([
    Buffer.from([0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a]),
    chunk("IHDR", ihdr),
    chunk("IDAT", zlib.deflateSync(raw, { level: 9 })),
    chunk("IEND", Buffer.alloc(0)),
  ]);
}

/** A 24-bit, bottom-up Windows BMP from an RGB image. */
export function bmp({ width, height, px }) {
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
      b[at + x * 3] = px[i + 2];
      b[at + x * 3 + 1] = px[i + 1];
      b[at + x * 3 + 2] = px[i];
    }
  }
  return b;
}

/** Read an ICO's directory: [{ width, height, data }] (PNG or BMP payloads as stored). */
export function readIco(buf) {
  if (buf.readUInt16LE(0) !== 0 || buf.readUInt16LE(2) !== 1) throw new Error("not an .ico file");
  const n = buf.readUInt16LE(4);
  return Array.from({ length: n }, (_, k) => {
    const e = 6 + k * 16;
    const size = buf.readUInt32LE(e + 8);
    const offset = buf.readUInt32LE(e + 12);
    return {
      width: buf[e] || 256,
      height: buf[e + 1] || 256,
      bitCount: buf.readUInt16LE(e + 6),
      data: buf.subarray(offset, offset + size),
    };
  });
}

/** Write an ICO from entries { width, height, data } (PNG payloads), smallest first. */
export function writeIco(entries) {
  const sorted = [...entries].sort((a, b) => a.width - b.width);
  const head = Buffer.alloc(6 + 16 * sorted.length);
  head.writeUInt16LE(0, 0);
  head.writeUInt16LE(1, 2);
  head.writeUInt16LE(sorted.length, 4);
  let offset = head.length;
  sorted.forEach((e, k) => {
    const at = 6 + k * 16;
    head[at] = e.width >= 256 ? 0 : e.width;
    head[at + 1] = e.height >= 256 ? 0 : e.height;
    head[at + 2] = 0; // palette
    head[at + 3] = 0;
    head.writeUInt16LE(1, at + 4); // planes
    head.writeUInt16LE(e.bitCount ?? 32, at + 6);
    head.writeUInt32LE(e.data.length, at + 8);
    head.writeUInt32LE(offset, at + 12);
    offset += e.data.length;
  });
  return Buffer.concat([head, ...sorted.map((e) => e.data)]);
}
