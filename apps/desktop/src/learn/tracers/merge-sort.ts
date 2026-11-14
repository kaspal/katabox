// Merge sort, top down. Mirrors content/learn/sorting/merge-sort/sample.{py,ts}: split the
// range in halves until one value is left, then merge each pair of sorted halves through
// a buffer (engine/merge.ts): the smaller head goes back first, the left one on ties, so
// the sort is stable.
import { SORT_INPUT } from "../engine/cases";
import { bufferedMerge } from "../engine/merge";
import { Recorder } from "../engine/recorder";
import type { NarrationKey, Step, TracerDef } from "../engine/types";

type Key = NarrationKey<"merge-sort">;
const KEYS = [
  "merge-sort.ready",
  "merge-sort.base",
  "merge-sort.split",
  "merge-sort.copy",
  "merge-sort.pick_left",
  "merge-sort.pick_right",
  "merge-sort.left_only",
  "merge-sort.right_only",
  "merge-sort.take",
  "merge-sort.done",
] as const satisfies readonly Key[];
const say = (key: (typeof KEYS)[number], params?: Record<string, number>) => ({ key, params });

export const MARKERS = ["base", "split", "copy", "pick", "take-left", "take-right"] as const;

function trace(values: readonly number[]): Step[] {
  const r = new Recorder(values);
  const n = values.length;
  const none = { lo: null, hi: null, mid: null, i: null, j: null, k: null };
  r.snap({ narration: say("merge-sort.ready", { n }), marker: null, vars: none });

  const sort = (lo: number, hi: number) => {
    if (lo >= hi) {
      r.snap({ range: [lo, hi], narration: say("merge-sort.base", { lo }), marker: "base", vars: { ...none, lo, hi } });
      return;
    }
    const mid = Math.floor((lo + hi) / 2);
    r.snap({
      range: [lo, hi],
      runs: [
        [lo, mid],
        [mid + 1, hi],
      ],
      narration: say("merge-sort.split", { lo, hi, mid, next: mid + 1 }),
      marker: "split",
      vars: { ...none, lo, hi, mid },
    });
    sort(lo, mid);
    sort(mid + 1, hi);
    bufferedMerge(r, lo, mid, hi, (e, f) => {
      const vars = { lo, hi, mid, ...f.vars };
      if (e.kind === "copy") {
        r.snap({ ...f, narration: say("merge-sort.copy", { lo, hi, mid }), marker: "copy", vars });
      } else if (e.kind === "pick") {
        const key =
          e.left === null
            ? "merge-sort.right_only"
            : e.right === null
              ? "merge-sort.left_only"
              : e.takeLeft
                ? "merge-sort.pick_left"
                : "merge-sort.pick_right";
        r.snap({ ...f, narration: say(key, { left: e.left ?? 0, right: e.right ?? 0 }), marker: "pick", vars });
      } else {
        r.snap({
          ...f,
          narration: say("merge-sort.take", { value: e.value, k: f.vars.k! }),
          marker: e.side === "left" ? "take-left" : "take-right",
          vars,
        });
      }
    });
  };
  sort(0, n - 1);
  r.settleAll();
  r.snap({ narration: say("merge-sort.done", { comparisons: r.comparisons, writes: r.writes, n }), marker: null });
  return r.steps;
}

export const mergeSort: TracerDef = {
  id: "merge-sort",
  subject: "sorting",
  compareGroup: "sort",
  scene: "merge",
  limits: SORT_INPUT,
  input: { kind: "values" },
  markers: MARKERS,
  narrationKeys: KEYS,
  counters: [
    { slot: "comparisons", label: "comparisons" },
    { slot: "writes", label: "writes" },
  ],
  legend: ["compare", "swap", "sorted", "out"],
  roleLabels: { swap: "writing" },
  vars: ["lo", "mid", "hi", "i", "j", "k"],
  trace,
};
