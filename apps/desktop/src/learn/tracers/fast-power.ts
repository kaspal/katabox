// Fast modular power by squaring. Mirrors content/learn/math-geometry/fast-power/sample.{py,ts}:
// one row per bit of the exponent. Output: [base^exp mod m].
import { Recorder } from "../engine/recorder";
import type { NarrationKey, Step, TracerDef } from "../engine/types";

type PowKey = NarrationKey<"fast-power">;
const POW_KEYS = ["fast-power.ready", "fast-power.multiply", "fast-power.square", "fast-power.done"] as const satisfies readonly PowKey[];

function powTrace(input: readonly number[]): Step[] {
  const [base0 = 1, exp0 = 0, mod = 2] = input;
  const r = new Recorder([]);
  const rows: (number | null)[][] = [];
  let result = 1 % mod;
  let base = base0 % mod;
  let exp = exp0;
  const say = (key: (typeof POW_KEYS)[number], params?: Record<string, number>) => ({ key, params });
  const snap = (narration: ReturnType<typeof say>, marker: string | null, hot: [number, number] | null = null) =>
    r.snap({
      narration,
      marker,
      dp: { label: "power_steps", colLabels: ["bit", "base", "result"], rowLabels: rows.map((_, k) => `2^${k}`), cells: rows.map((row) => [...row]), hot },
      vars: { exp, base, result },
    });
  snap(say("fast-power.ready", { base: base0, exp: exp0, mod }), null);
  let k = 0;
  while (exp > 0) {
    rows.push([exp & 1, base, null]);
    if (exp & 1) {
      result = (result * base) % mod;
      r.comparisons++;
      rows[k]![2] = result;
      snap(say("fast-power.multiply", { base, result }), "multiply", [k, 2]);
    } else rows[k]![2] = result;
    base = (base * base) % mod;
    r.writes++;
    snap(say("fast-power.square", { base }), "square", [k, 1]);
    exp >>= 1;
    k++;
  }
  snap(say("fast-power.done", { base: base0, exp: exp0, mod, result, multiplications: r.comparisons + r.writes }), null);
  r.steps[r.steps.length - 1]!.result = [result];
  return r.steps;
}

export const fastPower: TracerDef = {
  id: "fast-power",
  subject: "math-geometry",
  scene: "dp",
  limits: { min_len: 3, max_len: 3, min: 0, max: 9999 },
  input: {
    kind: "codec",
    codec: {
      fields: ["base", "exponent", "modulus"],
      format: (input) => input.map(String),
      parse: (texts) => {
        const out: number[] = [];
        for (const t of texts.slice(0, 3)) {
          const v = Number((t ?? "").trim());
          if (!/^\d+$/.test((t ?? "").trim())) return { error: { code: "custom", key: "whole_number" } };
          out.push(v);
        }
        return { values: out };
      },
      check: (input) => (input[2]! < 2 || input[2]! > 999 || input[0]! > 999 ? { code: "custom", key: "power_range" } : null),
    },
  },
  presets: {
    "big-exponent": () => [7, 1000, 13],
    random: (next) => [Math.floor(next() * 100), Math.floor(next() * 10000), 2 + Math.floor(next() * 998)],
    "power-of-two": () => [3, 64, 101],
  },
  markers: ["multiply", "square"],
  narrationKeys: POW_KEYS,
  counters: [
    { slot: "comparisons", label: "multiplications" },
    { slot: "writes", label: "squarings" },
  ],
  legend: ["key", "sorted"],
  roleLabels: { key: "current", sorted: "filled" },
  vars: ["exp", "base", "result"],
  trace: powTrace,
};
