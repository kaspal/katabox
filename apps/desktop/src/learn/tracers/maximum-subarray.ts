// Maximum subarray, Kadane's algorithm. Mirrors
// content/learn/dynamic-programming/maximum-subarray/sample.{py,ts}: walk left to right
// keeping `current`, the best sum of a run ending here; a run whose sum has dropped to 0
// or below cannot help what follows, so restart at the current value; otherwise extend.
// Remember the best run seen. Output: the best sum, its first and its last index.
import { Recorder } from "../engine/recorder";
import type { NarrationKey, Pointer, Role, Step, TracerDef } from "../engine/types";

type Key = NarrationKey<"maximum-subarray">;
const KEYS = ["maximum-subarray.ready", "maximum-subarray.restart", "maximum-subarray.extend", "maximum-subarray.record", "maximum-subarray.done"] as const satisfies readonly Key[];
const say = (key: (typeof KEYS)[number], params?: Record<string, number>) => ({ key, params });

export const MARKERS = ["restart", "extend", "record"] as const;

function trace(values: readonly number[]): Step[] {
  const r = new Recorder(values);
  let best = values[0]!;
  let bestStart = 0;
  let bestEnd = 0;
  let current = 0;
  let start = 0;
  const snap = (narration: ReturnType<typeof say>, marker: string | null, i: number | null) => {
    const roles: Record<number, Role> = {};
    for (let k = bestStart; k <= bestEnd; k++) roles[k] = "sorted";
    if (i !== null) for (let k = start; k <= i; k++) roles[k] = roles[k] === "sorted" ? "sorted" : "compare";
    if (i !== null) roles[i] = "key";
    const pointers: Pointer[] = i === null ? [] : [{ label: "start", at: start, tone: "i" }, { label: "i", at: i, tone: "key" }];
    r.snap({ roles, pointers, narration, marker, vars: { current: i === null ? null : current, best } });
  };

  snap(say("maximum-subarray.ready", { n: values.length }), null, null);
  values.forEach((value, i) => {
    r.comparisons++;
    if (current <= 0) {
      current = value;
      start = i;
      snap(say("maximum-subarray.restart", { i, value }), "restart", i);
    } else {
      current += value;
      snap(say("maximum-subarray.extend", { i, value, current }), "extend", i);
    }
    if (current > best) {
      best = current;
      bestStart = start;
      bestEnd = i;
      r.writes++;
      snap(say("maximum-subarray.record", { best, from: start, to: i }), "record", i);
    }
  });
  snap(say("maximum-subarray.done", { best, from: bestStart, to: bestEnd }), null, null);
  r.steps[r.steps.length - 1]!.result = [best, bestStart, bestEnd];
  return r.steps;
}

export const maximumSubarray: TracerDef = {
  id: "maximum-subarray",
  subject: "dynamic-programming",
  scene: "array",
  limits: { min_len: 1, max_len: 14, min: -9, max: 9 },
  input: { kind: "values" },
  presets: {
    mixed: () => [-2, 3, -1, 4, -6, 2, 5, -3, 1],
    random: (next) => Array.from({ length: 6 + Math.floor(next() * 8) }, () => Math.floor(next() * 19) - 9),
    "all-negative": () => [-4, -2, -7, -1, -5],
  },
  markers: MARKERS,
  narrationKeys: KEYS,
  counters: [
    { slot: "comparisons", label: "values_read" },
    { slot: "writes", label: "records" },
  ],
  legend: ["key", "compare", "sorted"],
  roleLabels: { key: "current", compare: "current_run", sorted: "best_run" },
  vars: ["current", "best"],
  trace,
};
