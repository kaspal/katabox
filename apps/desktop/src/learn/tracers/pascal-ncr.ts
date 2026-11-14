// Pascal's triangle. Mirrors content/learn/math-geometry/pascal-ncr/sample.{py,ts}: each
// inner entry is the sum of the two above. Output: row n, C(n, 0..n).
import { Recorder } from "../engine/recorder";
import type { NarrationKey, Step, TracerDef } from "../engine/types";

type PascalKey = NarrationKey<"pascal-ncr">;
const PASCAL_KEYS = ["pascal-ncr.ready", "pascal-ncr.add", "pascal-ncr.row", "pascal-ncr.done"] as const satisfies readonly PascalKey[];

function pascalTrace(input: readonly number[]): Step[] {
  const n = input[0]!;
  const r = new Recorder([]);
  const cells: (number | null)[][] = Array.from({ length: n + 1 }, () => new Array(n + 1).fill(null));
  cells[0]![0] = 1;
  const say = (key: (typeof PASCAL_KEYS)[number], params?: Record<string, number>) => ({ key, params });
  const snap = (narration: ReturnType<typeof say>, marker: string | null, hot: [number, number] | null = null, deps: [number, number][] = []) =>
    r.snap({
      narration,
      marker,
      dp: { label: "pascal", colLabels: Array.from({ length: n + 1 }, (_, k) => String(k)), rowLabels: Array.from({ length: n + 1 }, (_, k) => String(k)), cells: cells.map((row) => [...row]), hot, deps },
      vars: { n: hot ? hot[0] : null, k: hot ? hot[1] : null },
    });
  snap(say("pascal-ncr.ready", { n }), null);
  for (let i = 1; i <= n; i++) {
    cells[i]![0] = 1;
    cells[i]![i] = 1;
    snap(say("pascal-ncr.row", { i }), null, [i, 0]);
    for (let k = 1; k < i; k++) {
      cells[i]![k] = cells[i - 1]![k - 1]! + cells[i - 1]![k]!;
      r.writes++;
      snap(say("pascal-ncr.add", { n: i, k, value: cells[i]![k]! }), "add", [i, k], [
        [i - 1, k - 1],
        [i - 1, k],
      ]);
    }
  }
  snap(say("pascal-ncr.done", { n, sum: 2 ** n }), null);
  r.steps[r.steps.length - 1]!.result = cells[n]!.map((v) => v!);
  return r.steps;
}

export const pascalNcr: TracerDef = {
  id: "pascal-ncr",
  subject: "math-geometry",
  scene: "dp",
  limits: { min_len: 1, max_len: 1, min: 1, max: 12 },
  input: { kind: "count" },
  markers: ["add"],
  narrationKeys: PASCAL_KEYS,
  counters: [{ slot: "writes", label: "additions" }],
  legend: ["key", "compare", "sorted"],
  roleLabels: { key: "current", compare: "read", sorted: "filled" },
  vars: ["n", "k"],
  trace: pascalTrace,
};
