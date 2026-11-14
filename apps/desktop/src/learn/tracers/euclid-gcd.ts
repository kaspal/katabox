// Euclid's GCD. Mirrors content/learn/math-geometry/euclid-gcd/sample.{py,ts}: replace
// (a, b) by (b, a mod b) until b is 0. A table of the steps. Output: [gcd, divisions].
import { Recorder } from "../engine/recorder";
import type { NarrationKey, NarrationParams, Step, TracerDef } from "../engine/types";

type GcdKey = NarrationKey<"euclid-gcd">;
const GCD_KEYS = ["euclid-gcd.ready", "euclid-gcd.divide", "euclid-gcd.done"] as const satisfies readonly GcdKey[];

function gcdTrace(input: readonly number[]): Step[] {
  let [a = 1, b = 1] = input;
  const r = new Recorder([]);
  const rows: (number | null)[][] = [];
  const say = (key: (typeof GCD_KEYS)[number], params?: NarrationParams<number>) => ({ key, params });
  const snap = (narration: ReturnType<typeof say>, marker: string | null) =>
    r.snap({
      narration,
      marker,
      dp: { label: "euclid_steps", colLabels: ["a", "b", "a mod b"], rowLabels: rows.map((_, k) => String(k)), cells: rows.map((row) => [...row]), hot: rows.length ? [rows.length - 1, 2] : null, deps: rows.length ? [[rows.length - 1, 0], [rows.length - 1, 1]] : [] },
      vars: { a, b },
    });
  snap(say("euclid-gcd.ready", { a, b }), null);
  let steps = 0;
  while (b !== 0) {
    rows.push([a, b, a % b]);
    [a, b] = [b, a % b];
    steps++;
    r.comparisons++;
    snap(say("euclid-gcd.divide", { a: rows[rows.length - 1]![0]!, b: rows[rows.length - 1]![1]!, rest: b }), "divide");
  }
  rows.push([a, 0, null]);
  snap(say("euclid-gcd.done", { gcd: a, steps }), null);
  r.steps[r.steps.length - 1]!.result = [a, steps];
  return r.steps;
}

export const euclidGcd: TracerDef = {
  id: "euclid-gcd",
  subject: "math-geometry",
  scene: "dp",
  limits: { min_len: 2, max_len: 2, min: 1, max: 999 },
  input: { kind: "values" },
  presets: {
    "big-factor": () => [336, 252],
    random: (next) => [1 + Math.floor(next() * 999), 1 + Math.floor(next() * 999)],
    "fibonacci-pair": () => [610, 377],
  },
  markers: ["divide"],
  narrationKeys: GCD_KEYS,
  counters: [{ slot: "comparisons", label: "divisions" }],
  legend: ["key", "compare", "sorted"],
  roleLabels: { key: "current", compare: "read", sorted: "filled" },
  vars: ["a", "b"],
  trace: gcdTrace,
};
