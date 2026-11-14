// Tim sort, simplified. Mirrors content/learn/sorting/tim-sort/sample.{py,ts}: insertion
// sort makes sorted runs of 4, then runs are merged in pairs, doubling the width, through
// the same buffered merge as merge sort (engine/merge.ts). Real Tim sort also finds runs
// already in the data, picks the run length from n and gallops; the theory says so.
import { SORT_INPUT } from "../engine/cases";
import { bufferedMerge } from "../engine/merge";
import { Recorder } from "../engine/recorder";
import type { NarrationKey, NarrationParams, Pointer, Step, TracerDef } from "../engine/types";

type Key = NarrationKey<"tim-sort">;
const KEYS = [
  "tim-sort.ready",
  "tim-sort.run",
  "tim-sort.take",
  "tim-sort.compare_bigger",
  "tim-sort.compare_stop",
  "tim-sort.shift",
  "tim-sort.insert",
  "tim-sort.merge",
  "tim-sort.copy",
  "tim-sort.pick_left",
  "tim-sort.pick_right",
  "tim-sort.left_only",
  "tim-sort.right_only",
  "tim-sort.take_value",
  "tim-sort.done",
] as const satisfies readonly Key[];
const say = (key: (typeof KEYS)[number], params?: NarrationParams<number>) => ({ key, params });

export const MARKERS = ["run", "take", "compare", "shift", "insert", "merge", "copy", "pick", "take-left", "take-right"] as const;
export const RUN = 4;

function trace(values: readonly number[]): Step[] {
  const r = new Recorder(values);
  const n = values.length;
  const none = { lo: null, hi: null, i: null, j: null, key: null, k: null };
  r.snap({ narration: say("tim-sort.ready", { n, run: RUN }), marker: null, vars: none });

  for (let start = 0; start < n; start += RUN) {
    const lo = start;
    const hi = Math.min(start + RUN, n) - 1;
    const range: [number, number] = [lo, hi];
    r.snap({ range, narration: say("tim-sort.run", { lo, hi }), marker: "run", vars: { ...none, lo, hi } });
    for (let i = lo + 1; i <= hi; i++) {
      const key = r.value(i);
      let gap = i;
      const ptrs = (j: number | null): Pointer[] => [
        { label: "i", at: i, tone: "i" },
        ...(j === null ? [] : [{ label: "j", at: j, tone: "j" } as Pointer]),
      ];
      r.snap({ roles: { [gap]: "key" }, lifted: gap, pointers: ptrs(null), range, narration: say("tim-sort.take", { key, i }), marker: "take", vars: { ...none, lo, hi, i, j: i - 1, key } });
      let j = i - 1;
      while (j >= lo) {
        r.comparisons++;
        const value = r.value(j);
        const bigger = value > key;
        r.snap({
          roles: { [j]: "compare", [gap]: "key" },
          lifted: gap,
          pointers: ptrs(j),
          range,
          narration: say(bigger ? "tim-sort.compare_bigger" : "tim-sort.compare_stop", { value, key }),
          marker: "compare",
          vars: { ...none, lo, hi, i, j, key },
        });
        if (!bigger) break;
        r.swap(j, gap);
        r.moves++;
        r.writes++;
        gap = j;
        r.snap({
          roles: { [j + 1]: "swap", [gap]: "key" },
          lifted: gap,
          pointers: ptrs(j),
          range,
          narration: say("tim-sort.shift", { value, to: j + 1 }),
          marker: "shift",
          vars: { ...none, lo, hi, i, j, key },
        });
        j--;
      }
      r.writes++;
      r.snap({ roles: { [gap]: "swap" }, pointers: ptrs(null), range, narration: say("tim-sort.insert", { key, at: gap }), marker: "insert", vars: { ...none, lo, hi, i, j, key } });
    }
  }

  for (let width = RUN; width < n; width *= 2) {
    for (let lo = 0; lo < n - width; lo += 2 * width) {
      const mid = lo + width - 1;
      const hi = Math.min(lo + 2 * width, n) - 1;
      r.snap({
        range: [lo, hi],
        runs: [
          [lo, mid],
          [mid + 1, hi],
        ],
        narration: say("tim-sort.merge", { lo, mid, hi, width }),
        marker: "merge",
        vars: { ...none, lo, hi },
      });
      bufferedMerge(r, lo, mid, hi, (e, f) => {
        const vars = { ...none, lo, hi, i: f.vars.i, j: f.vars.j, k: f.vars.k };
        if (e.kind === "copy") {
          r.snap({ ...f, narration: say("tim-sort.copy", { lo, hi }), marker: "copy", vars });
        } else if (e.kind === "pick") {
          const key =
            e.left === null ? "tim-sort.right_only" : e.right === null ? "tim-sort.left_only" : e.takeLeft ? "tim-sort.pick_left" : "tim-sort.pick_right";
          r.snap({ ...f, narration: say(key, { left: e.left ?? 0, right: e.right ?? 0 }), marker: "pick", vars });
        } else {
          r.snap({ ...f, narration: say("tim-sort.take_value", { value: e.value, k: f.vars.k! }), marker: e.side === "left" ? "take-left" : "take-right", vars });
        }
      });
    }
  }
  r.settleAll();
  r.snap({ narration: say("tim-sort.done", { comparisons: r.comparisons, writes: r.writes, n }), marker: null });
  return r.steps;
}

export const timSort: TracerDef = {
  id: "tim-sort",
  subject: "sorting",
  compareGroup: "sort",
  scene: "merge",
  lifts: true,
  limits: SORT_INPUT,
  input: { kind: "values" },
  markers: MARKERS,
  narrationKeys: KEYS,
  counters: [
    { slot: "comparisons", label: "comparisons" },
    { slot: "writes", label: "writes" },
  ],
  legend: ["key", "compare", "swap", "sorted", "out"],
  roleLabels: { swap: "writing" },
  vars: ["lo", "hi", "i", "j", "key", "k"],
  trace,
};
