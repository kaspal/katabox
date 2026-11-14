// Longest increasing subsequence, O(n²). Mirrors content/learn/dynamic-programming/lis/sample.{py,ts}:
// length[i] is the longest increasing run (not necessarily adjacent) that ends at value
// i: 1, or one more than the best length[j] of a smaller value before it. prev[] remembers
// the j, so the chain can be read back from the best end (the first one, on ties).
// Output: the length, then the subsequence.
import { Recorder } from "../engine/recorder";
import type { NarrationKey, NarrationParams, Role, Step, TracerDef } from "../engine/types";

type Key = NarrationKey<"lis">;
const KEYS = ["lis.ready", "lis.start", "lis.extend", "lis.trace", "lis.done"] as const satisfies readonly Key[];
const say = (key: (typeof KEYS)[number], params?: NarrationParams<number>) => ({ key, params });

export const MARKERS = ["extend", "trace"] as const;

function trace(values: readonly number[]): Step[] {
  const n = values.length;
  const r = new Recorder([]);
  const length: (number | null)[] = new Array(n).fill(null);
  const prev = new Array<number>(n).fill(-1);
  const chain: number[] = [];
  const path: [number, number][] = [];
  const snap = (narration: ReturnType<typeof say>, marker: string | null, i: number | null = null, j: number | null = null) => {
    const roles: Record<number, Role> = {};
    for (const [, k] of path) roles[k] = "swap";
    if (j !== null) roles[j] = "compare";
    if (i !== null) roles[i] = "key";
    r.snap({
      narration,
      marker,
      slots: { layout: "row", label: "values", cells: values.map((value, id) => ({ id, value })), roles },
      dp: { label: "lis_length", colLabels: values.map((_, k) => String(k)), cells: [[...length]], hot: i === null ? null : [0, i], deps: j === null ? [] : [[0, j]], path: [...path] },
      vars: { i, j, length: i === null ? null : length[i] },
    });
  };

  snap(say("lis.ready", { n }), null);
  for (let i = 0; i < n; i++) {
    length[i] = 1;
    snap(say("lis.start", { i, value: values[i]! }), null, i);
    for (let j = 0; j < i; j++) {
      r.comparisons++;
      if (values[j]! < values[i]! && length[j]! + 1 > length[i]!) {
        length[i] = length[j]! + 1;
        prev[i] = j;
        r.writes++;
        snap(say("lis.extend", { i, j, a: values[j]!, b: values[i]!, length: length[i]! }), "extend", i, j);
      }
    }
  }
  let end = 0;
  for (let k = 1; k < n; k++) if (length[k]! > length[end]!) end = k;
  while (end !== -1) {
    chain.push(values[end]!);
    path.push([0, end]);
    snap(say("lis.trace", { k: end, value: values[end]! }), "trace");
    end = prev[end]!;
  }
  chain.reverse();
  snap(say("lis.done", { length: chain.length }), null);
  r.steps[r.steps.length - 1]!.result = [chain.length, ...chain];
  return r.steps;
}

export const lis: TracerDef = {
  id: "lis",
  subject: "dynamic-programming",
  scene: "dp",
  limits: { min_len: 1, max_len: 12, min: 1, max: 99 },
  input: { kind: "values" },
  markers: MARKERS,
  narrationKeys: KEYS,
  counters: [
    { slot: "comparisons", label: "comparisons" },
    { slot: "writes", label: "writes" },
  ],
  legend: ["key", "compare", "sorted", "swap"],
  roleLabels: { key: "current", compare: "read", sorted: "filled", swap: "taken" },
  vars: ["i", "j", "length"],
  trace,
};
