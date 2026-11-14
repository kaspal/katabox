// House robber: the most loot from houses in a row without taking two neighbours.
// Mirrors content/learn/dynamic-programming/house-robber/sample.{py,ts}: best[i] is the
// most from the first i houses; house i − 1 is either skipped (best[i − 1]) or taken
// (best[i − 2] + its value). Tracing back from the end recovers the houses taken.
// Output: the best total, then the houses taken (0-based, left to right).
import { Recorder } from "../engine/recorder";
import type { NarrationKey, NarrationParams, Role, Step, TracerDef } from "../engine/types";

type Key = NarrationKey<"house-robber">;
const KEYS = ["house-robber.ready", "house-robber.first", "house-robber.take", "house-robber.skip", "house-robber.trace", "house-robber.done"] as const satisfies readonly Key[];
const say = (key: (typeof KEYS)[number], params?: NarrationParams<number>) => ({ key, params });

export const MARKERS = ["choose", "trace"] as const;

function trace(values: readonly number[]): Step[] {
  const n = values.length;
  const r = new Recorder([]);
  const best: (number | null)[] = new Array(n + 1).fill(null);
  best[0] = 0;
  const chosen: number[] = [];
  const path: [number, number][] = [];
  const snap = (narration: ReturnType<typeof say>, marker: string | null, hot: number | null = null, deps: number[] = [], house: number | null = null) => {
    const roles: Record<number, Role> = {};
    for (const h of chosen) roles[h] = "swap";
    if (house !== null) roles[house] = "key";
    r.snap({
      narration,
      marker,
      slots: { layout: "row", label: "houses", cells: values.map((value, id) => ({ id, value })), roles },
      dp: { label: "best", colLabels: Array.from({ length: n + 1 }, (_, k) => String(k)), cells: [[...best]], hot: hot === null ? null : [0, hot], deps: deps.map((d) => [0, d]), path: [...path] },
      vars: { i: hot, best: hot === null ? null : best[hot]! },
    });
  };

  snap(say("house-robber.ready", { n }), null);
  best[1] = values[0]!;
  r.writes++;
  snap(say("house-robber.first", { value: values[0]! }), null, 1, [], 0);
  for (let i = 2; i <= n; i++) {
    const skip = best[i - 1]!;
    const take = best[i - 2]! + values[i - 1]!;
    best[i] = Math.max(skip, take);
    r.comparisons++;
    r.writes++;
    snap(take > skip ? say("house-robber.take", { i, take, skip, house: i - 1 }) : say("house-robber.skip", { i, take, skip, house: i - 1 }), "choose", i, [i - 1, i - 2], i - 1);
  }
  let i = n;
  while (i > 0) {
    if (best[i] === best[i - 1]) i -= 1;
    else {
      chosen.push(i - 1);
      path.push([0, i]);
      snap(say("house-robber.trace", { house: i - 1, value: values[i - 1]! }), "trace", null, [], i - 1);
      i -= 2;
    }
  }
  chosen.reverse();
  snap(say("house-robber.done", { best: best[n]!, num: chosen.length }), null);
  r.steps[r.steps.length - 1]!.result = [best[n]!, ...chosen];
  return r.steps;
}

export const houseRobber: TracerDef = {
  id: "house-robber",
  subject: "dynamic-programming",
  scene: "dp",
  limits: { min_len: 1, max_len: 12, min: 1, max: 50 },
  input: { kind: "values" },
  markers: MARKERS,
  narrationKeys: KEYS,
  counters: [
    { slot: "comparisons", label: "comparisons" },
    { slot: "writes", label: "writes" },
  ],
  legend: ["key", "compare", "sorted", "swap"],
  roleLabels: { key: "current", compare: "read", sorted: "filled", swap: "taken" },
  vars: ["i", "best"],
  trace,
};
