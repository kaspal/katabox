// Shared by the plane topics (ADR 0029, Phase 10): convex-hull.ts and
// segment-intersection.ts. Points, the cross product and the points codec.
import { fail } from "../engine/codecs";
import { tokens, type InputError } from "../engine/inputs";
import type { InputCodec } from "../engine/types";

export type P = [number, number];
export const MAX_COORD = 20;
export const cross = (o: P, a: P, b: P) => (a[0] - o[0]) * (b[1] - o[1]) - (a[1] - o[1]) * (b[0] - o[0]);
export const pts = (input: readonly number[]) => Array.from({ length: Math.floor(input.length / 2) }, (_, k) => [input[2 * k]!, input[2 * k + 1]!] as P);

/** Points typed as "x y" pairs. */
export function pointsCodec(o: { min: number; max: number; field?: "points" | "segments" }): InputCodec {
  const check = (input: readonly number[]): InputError | null => {
    if (input.length % 2 !== 0) return { code: "custom", key: "point_pair", params: { item: "" } };
    const n = input.length / 2;
    if (n < o.min || n > o.max) return { code: "custom", key: "point_count", params: { min: o.min, max: o.max } };
    if (input.some((v) => v < 0 || v > MAX_COORD)) return { code: "custom", key: "coordinate_range", params: { max: MAX_COORD } };
    return null;
  };
  return {
    fields: [o.field ?? "points"],
    format: (input) => [pts(input).map(([x, y]) => `${x} ${y}`).join(", ")],
    parse: (texts) => {
      const out: number[] = [];
      for (const chunk of (texts[0] ?? "").split(/[,;\n]+/).map((s) => s.trim()).filter((s) => s !== "")) {
        const t = tokens(chunk);
        if ("error" in t) return t;
        if (t.values.length !== 2) return fail("point_pair", { item: chunk });
        out.push(...t.values);
      }
      const e = check(out);
      return e ? { error: e } : { values: out };
    },
    check,
  };
}
