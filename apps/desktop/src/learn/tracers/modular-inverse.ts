// Modular inverse by the extended Euclidean algorithm. Mirrors
// content/learn/math-geometry/modular-inverse/sample.{py,ts}. Output: [inverse] or [-1].
import { Recorder } from "../engine/recorder";
import type { NarrationKey, Step, TracerDef } from "../engine/types";

type InvKey = NarrationKey<"modular-inverse">;
const INV_KEYS = ["modular-inverse.ready", "modular-inverse.divide", "modular-inverse.done", "modular-inverse.none"] as const satisfies readonly InvKey[];

function invTrace(input: readonly number[]): Step[] {
  const [a = 1, m = 2] = input;
  const r = new Recorder([]);
  let [oldR, rr] = [a % m, m];
  let [oldX, x] = [1, 0];
  const rows: (number | null)[][] = [[oldR, oldX, null]];
  const say = (key: (typeof INV_KEYS)[number], params?: Record<string, number>) => ({ key, params });
  const snap = (narration: ReturnType<typeof say>, marker: string | null) =>
    r.snap({
      narration,
      marker,
      dp: { label: "extended_euclid", colLabels: ["r", "x", "q"], rowLabels: rows.map((_, k) => String(k)), cells: rows.map((row) => [...row]), hot: [rows.length - 1, 0], deps: rows.length > 2 ? [[rows.length - 2, 0], [rows.length - 3, 0]] : [] },
      vars: { r: oldR, x: oldX },
    });
  rows.push([rr, x, null]);
  snap(say("modular-inverse.ready", { a, m }), null);
  while (rr !== 0) {
    const q = Math.floor(oldR / rr);
    rows[rows.length - 1]![2] = q;
    [oldR, rr] = [rr, oldR - q * rr];
    [oldX, x] = [x, oldX - q * x];
    rows.push([rr, x, null]);
    r.comparisons++;
    snap(say("modular-inverse.divide", { q, r: rr, x }), "divide");
  }
  if (oldR !== 1) {
    snap(say("modular-inverse.none", { a, m, g: oldR }), null);
    r.steps[r.steps.length - 1]!.result = [-1];
    return r.steps;
  }
  const inv = ((oldX % m) + m) % m;
  snap(say("modular-inverse.done", { a, m, inv }), null);
  r.steps[r.steps.length - 1]!.result = [inv];
  return r.steps;
}

export const modularInverse: TracerDef = {
  id: "modular-inverse",
  subject: "math-geometry",
  scene: "dp",
  limits: { min_len: 2, max_len: 2, min: 1, max: 999 },
  input: {
    kind: "codec",
    codec: {
      fields: ["number", "modulus"],
      format: (input) => input.map(String),
      parse: (texts) => {
        const out: number[] = [];
        for (const t of texts.slice(0, 2)) {
          if (!/^\d+$/.test((t ?? "").trim())) return { error: { code: "custom", key: "whole_number" } };
          out.push(Number(t!.trim()));
        }
        return { values: out };
      },
      check: (input) => (input[1]! < 2 ? { code: "custom", key: "power_range" } : null),
    },
  },
  presets: {
    "prime-modulus": () => [17, 101],
    random: (next) => [1 + Math.floor(next() * 999), 2 + Math.floor(next() * 998)],
    "shares-a-factor": () => [12, 30],
  },
  markers: ["divide"],
  narrationKeys: INV_KEYS,
  counters: [{ slot: "comparisons", label: "divisions" }],
  legend: ["key", "compare", "sorted"],
  roleLabels: { key: "current", compare: "read", sorted: "filled" },
  vars: ["r", "x"],
  trace: invTrace,
};
