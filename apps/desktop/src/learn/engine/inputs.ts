// Inputs for a topic: seeded presets and the "your own" parser with clear errors.
import type { LearnInput } from "@/bindings";

export const PRESETS = ["random", "nearly-sorted", "reversed", "few-unique"] as const;
export type PresetId = (typeof PRESETS)[number];
export const isPreset = (s: string): s is PresetId => (PRESETS as readonly string[]).includes(s);

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

const int = (next: () => number, lo: number, hi: number) => lo + Math.floor(next() * (hi - lo + 1));

export function randomValues(seed: number, len: number, lim: LearnInput): number[] {
  const next = rng(seed);
  return Array.from({ length: len }, () => int(next, Math.max(lim.min, 3), lim.max));
}

/** Preset `id` with `len` values (clamped to the topic's limits), from `seed`. */
export function preset(id: PresetId, seed: number, len: number, lim: LearnInput): number[] {
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
      const span = lim.max - Math.max(lim.min, 3);
      const pool = [0.2, 0.45, 0.7].map((f) => Math.max(lim.min, 3) + Math.round(f * span));
      return Array.from({ length: n }, () => pool[int(next, 0, pool.length - 1)]!);
    }
  }
}

export type InputError =
  | { code: "count"; min: number; max: number }
  | { code: "nan"; token: string }
  | { code: "range"; value: number; min: number; max: number };

/** Parse "your own" input: integers separated by commas, spaces or semicolons. */
export function parseCustom(text: string, lim: LearnInput): { values: number[] } | { error: InputError } {
  const tokens = text.split(/[\s,;]+/).filter((t) => t !== "");
  for (const token of tokens) {
    if (!/^-?\d+$/.test(token)) return { error: { code: "nan", token } };
  }
  const values = tokens.map(Number);
  if (values.length < lim.min_len || values.length > lim.max_len) {
    return { error: { code: "count", min: lim.min_len, max: lim.max_len } };
  }
  const bad = values.find((v) => v < lim.min || v > lim.max);
  if (bad !== undefined) return { error: { code: "range", value: bad, min: lim.min, max: lim.max } };
  return { values };
}
