// Matrix chain multiplication (interval DP). Mirrors
// content/learn/dynamic-programming/matrix-chain/sample.{py,ts}: matrix k is dims[k] ×
// dims[k + 1]; cost[i][j] is the fewest scalar multiplications for matrices i..j. It
// fills by span (2 matrices, then 3, …), trying every split point k: left part, right
// part, and dims[i] · dims[k + 1] · dims[j + 1] to multiply the two results. The table
// fills diagonal by diagonal. Output: [the cheapest cost for the whole chain].
import { Recorder } from "../engine/recorder";
import type { NarrationKey, Step, TracerDef } from "../engine/types";

type Key = NarrationKey<"matrix-chain">;
const KEYS = ["matrix-chain.ready", "matrix-chain.span", "matrix-chain.split", "matrix-chain.worse", "matrix-chain.done"] as const satisfies readonly Key[];
const say = (key: (typeof KEYS)[number], params?: Record<string, number>) => ({ key, params });

export const MARKERS = ["split"] as const;

function trace(dims: readonly number[]): Step[] {
  const n = dims.length - 1;
  const r = new Recorder([]);
  const cost: (number | null)[][] = Array.from({ length: n }, (_, i) => Array.from({ length: n }, (_, j) => (i === j ? 0 : null)));
  const snap = (narration: ReturnType<typeof say>, marker: string | null, hot: [number, number] | null = null, deps: [number, number][] = []) =>
    r.snap({
      narration,
      marker,
      slots: { layout: "row", label: "matrices", cells: Array.from({ length: n }, (_, k) => ({ id: k, value: k, text: `${dims[k]}×${dims[k + 1]}` })), roles: hot ? Object.fromEntries(Array.from({ length: hot[1] - hot[0] + 1 }, (_, k) => [hot[0] + k, "compare"])) : {} },
      dp: { label: "multiplications", colLabels: Array.from({ length: n }, (_, k) => String(k)), rowLabels: Array.from({ length: n }, (_, k) => String(k)), cells: cost.map((row) => [...row]), hot, deps },
      vars: { i: hot ? hot[0] : null, j: hot ? hot[1] : null },
    });

  snap(say("matrix-chain.ready", { n }), null);
  for (let span = 2; span <= n; span++) {
    snap(say("matrix-chain.span", { span }), null);
    for (let i = 0; i + span - 1 < n; i++) {
      const j = i + span - 1;
      cost[i]![j] = 1e9;
      for (let k = i; k < j; k++) {
        r.comparisons++;
        const join = dims[i]! * dims[k + 1]! * dims[j + 1]!;
        const total = cost[i]![k]! + cost[k + 1]![j]! + join;
        if (total < cost[i]![j]!) {
          cost[i]![j] = total;
          r.writes++;
          snap(say("matrix-chain.split", { i, j, k, left: cost[i]![k]!, right: cost[k + 1]![j]!, join, total }), "split", [i, j], [
            [i, k],
            [k + 1, j],
          ]);
        } else {
          snap(say("matrix-chain.worse", { i, j, k, total, best: cost[i]![j]! }), null, [i, j], [
            [i, k],
            [k + 1, j],
          ]);
        }
      }
    }
  }
  snap(say("matrix-chain.done", { n, total: cost[0]![n - 1]! }), null);
  r.steps[r.steps.length - 1]!.result = [cost[0]![n - 1]!];
  return r.steps;
}

export const matrixChain: TracerDef = {
  id: "matrix-chain",
  subject: "dynamic-programming",
  scene: "dp",
  limits: { min_len: 3, max_len: 7, min: 1, max: 30 },
  input: { kind: "values" },
  presets: {
    "order-matters": () => [8, 20, 4, 30],
    random: (next) => Array.from({ length: 3 + Math.floor(next() * 5) }, () => 2 + Math.floor(next() * 28)),
    "long-chain": () => [5, 10, 3, 12, 5, 25, 6],
  },
  markers: MARKERS,
  narrationKeys: KEYS,
  counters: [
    { slot: "comparisons", label: "comparisons" },
    { slot: "writes", label: "writes" },
  ],
  legend: ["key", "compare", "sorted"],
  roleLabels: { key: "current", compare: "read", sorted: "filled" },
  vars: ["i", "j"],
  trace,
};
