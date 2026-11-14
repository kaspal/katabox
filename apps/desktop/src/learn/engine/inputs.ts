// Inputs for a topic: seeded presets and the "your own" parser with clear errors. Every
// input is one flat list of integers; target kinds put the target first (types.ts).
import type { LearnInput } from "@/bindings";
import type { TracerDef } from "./types";

/** The four presets of value topics (sorts, searches). */
export const PRESETS = ["random", "nearly-sorted", "reversed", "few-unique"] as const;
type SortPreset = (typeof PRESETS)[number];
/** A preset id: one of PRESETS, or a topic's own (TracerDef.presets). */
export type PresetId = string;
const isSortPreset = (s: string): s is SortPreset => (PRESETS as readonly string[]).includes(s);

/** The presets a topic can offer, in order. */
export function presetIds(def: TracerDef): string[] {
  return def.presets ? Object.keys(def.presets) : [...PRESETS];
}

/** mulberry32: small, fast and the same on every machine (cases.json depends on it). */
export function rng(seed: number): () => number {
  let a = seed >>> 0;
  return () => {
    a = (a + 0x6d2b79f5) >>> 0;
    let t = a;
    t = Math.imul(t ^ (t >>> 15), t | 1);
    t ^= t + Math.imul(t ^ (t >>> 7), t | 61);
    return ((t ^ (t >>> 14)) >>> 0) / 4294967296;
  };
}

export const int = (next: () => number, lo: number, hi: number) => lo + Math.floor(next() * (hi - lo + 1));

/** The smallest value presets use: tiny bars are hard to see. */
const floor = (lim: LearnInput) => (lim.max - lim.min >= 20 ? Math.max(lim.min, 3) : lim.min);

export function randomValues(seed: number, len: number, lim: LearnInput): number[] {
  const next = rng(seed);
  return Array.from({ length: len }, () => int(next, floor(lim), lim.max));
}

/** Preset `id` with `len` values (clamped to the topic's limits), from `seed`. */
export function preset(id: SortPreset, seed: number, len: number, lim: LearnInput): number[] {
  const n = Math.min(Math.max(len, lim.min_len), lim.max_len);
  const next = rng(seed);
  switch (id) {
    case "random":
      return randomValues(seed, n, lim);
    case "nearly-sorted": {
      const xs = randomValues(seed, n, lim).sort((a, b) => a - b);
      // Swap two pairs a few places apart: a couple of values out of place.
      for (let k = 0; k < Math.max(1, Math.floor(n / 6)); k++) {
        const a = int(next, 0, n - 2);
        const b = Math.min(n - 1, a + 1 + int(next, 0, 2));
        [xs[a], xs[b]] = [xs[b]!, xs[a]!];
      }
      return xs;
    }
    case "reversed":
      return randomValues(seed, n, lim).sort((a, b) => b - a);
    case "few-unique": {
      const lo = floor(lim);
      const span = lim.max - lo;
      const pool = [0.2, 0.45, 0.7].map((f) => lo + Math.round(f * span));
      return Array.from({ length: n }, () => pool[int(next, 0, pool.length - 1)]!);
    }
  }
}

/** The limits of the values part (target kinds hold one number fewer). */
export function valueLimits(def: TracerDef): LearnInput {
  const lim = def.limits;
  if (def.input.kind === "target" || def.input.kind === "sorted-target") {
    return { ...lim, min_len: Math.max(1, lim.min_len - 1), max_len: lim.max_len - 1 };
  }
  return lim;
}

export const hasTarget = (def: TracerDef) => def.input.kind === "target" || def.input.kind === "sorted-target";

/** A preset input for the topic: `len` is the length of the whole flat input. */
export function presetInput(def: TracerDef, id: PresetId, seed: number, len: number): number[] {
  const lim = def.limits;
  if (def.presets) {
    const make = def.presets[id] ?? Object.values(def.presets)[0]!;
    return make(rng(seed));
  }
  const sp: SortPreset = isSortPreset(id) ? id : "random";
  if (def.input.kind === "count") return [int(rng(seed), lim.min, lim.max)];
  if (!hasTarget(def)) return preset(sp, seed, len, lim);
  const values = preset(sp, seed, len - 1, valueLimits(def));
  if (def.input.kind === "sorted-target") values.sort((a, b) => a - b);
  const next = rng(seed ^ 0x5bd1e995);
  const target = def.input.pickTarget ? def.input.pickTarget(values, next) : values[int(next, 0, values.length - 1)]!;
  return [target, ...values];
}

export type InputError =
  | { code: "count"; min: number; max: number }
  | { code: "nan"; token: string }
  | { code: "range"; value: number; min: number; max: number }
  | { code: "unsorted" }
  | { code: "target"; min: number; max: number }
  /** A structured input that cannot be read or does not make sense
   * (`learn.input.errors.custom.<key>`). */
  | { code: "custom"; key: string; params?: Record<string, string | number> };

/** Check a whole flat input against the topic's limits and shape. */
export function checkInput(def: TracerDef, input: readonly number[]): InputError | null {
  const lim = def.limits;
  if (def.input.kind === "codec") {
    if (input.length < lim.min_len || input.length > lim.max_len) {
      return { code: "custom", key: "size" };
    }
    const out = input.find((v) => v < lim.min || v > lim.max);
    if (out !== undefined) return { code: "range", value: out, min: lim.min, max: lim.max };
    return def.input.codec!.check?.(input) ?? null;
  }
  const vl = valueLimits(def);
  const values = hasTarget(def) ? input.slice(1) : input;
  if (hasTarget(def) && input.length === 0) return { code: "count", min: vl.min_len, max: vl.max_len };
  if (values.length < vl.min_len || values.length > vl.max_len) {
    return { code: "count", min: vl.min_len, max: vl.max_len };
  }
  const bad = input.find((v) => v < lim.min || v > lim.max);
  if (bad !== undefined) return { code: "range", value: bad, min: lim.min, max: lim.max };
  if (def.input.kind === "sorted-target" && values.some((v, k) => k > 0 && v < values[k - 1]!)) {
    return { code: "unsorted" };
  }
  if (hasTarget(def) && def.input.checkTarget) {
    const e = def.input.checkTarget(input[0]!, values);
    if (e) return { code: "target", ...e };
  }
  return null;
}

/** Integers separated by commas, spaces or semicolons. */
export function tokens(text: string): { values: number[] } | { error: InputError } {
  const parts = text.split(/[\s,;]+/).filter((t) => t !== "");
  for (const token of parts) {
    if (!/^-?\d+$/.test(token)) return { error: { code: "nan", token } };
  }
  return { values: parts.map(Number) };
}

/** The "your own" fields of a topic with their texts for `input`. */
export function fieldsOf(def: TracerDef, input: readonly number[]): { key: string; text: string }[] {
  switch (def.input.kind) {
    case "codec": {
      const texts = def.input.codec!.format(input);
      return def.input.codec!.fields.map((key, k) => ({ key, text: texts[k] ?? "" }));
    }
    case "count":
      return [{ key: "count", text: String(input[0] ?? "") }];
    case "target":
    case "sorted-target":
      return [
        { key: "values", text: input.slice(1).join(", ") },
        { key: def.input.targetLabel ?? "target", text: String(input[0] ?? "") },
      ];
    default:
      return [{ key: "values", text: input.join(", ") }];
  }
}

/** Parse "your own" input: one text per field (see `fieldsOf`). */
export function parseCustom(def: TracerDef, texts: readonly string[]): { values: number[] } | { error: InputError } {
  if (def.input.kind === "codec") {
    const r = def.input.codec!.parse(texts);
    if ("error" in r) return r;
    const e = checkInput(def, r.values);
    return e ? { error: e } : r;
  }
  const text = texts[0] ?? "";
  const targetText = texts[1] ?? "";
  const r = tokens(text);
  if ("error" in r) return r;
  let input = r.values;
  if (hasTarget(def)) {
    const tr = tokens(targetText);
    if ("error" in tr) return tr;
    if (tr.values.length !== 1) return { error: { code: "nan", token: targetText.trim() || "—" } };
    input = [tr.values[0]!, ...input];
  }
  const e = checkInput(def, input);
  return e ? { error: e } : { values: input };
}
