// Quick sort, Lomuto partition with the last value as pivot. Mirrors
// content/learn/sorting/quick-sort/sample.{py,ts} line for line (`@step` markers).
import { Recorder } from "../engine/recorder";
import type { NarrationKey, Pointer, Step, TracerDef } from "../engine/types";

type Key = NarrationKey<"quick-sort">;
const KEYS = [
  "quick-sort.ready",
  "quick-sort.base",
  "quick-sort.pivot",
  "quick-sort.compare_smaller",
  "quick-sort.compare_not",
  "quick-sort.swap",
  "quick-sort.swap_self",
  "quick-sort.place",
  "quick-sort.done",
] as const satisfies readonly Key[];
const say = (key: (typeof KEYS)[number], params?: Record<string, number>) => ({ key, params });

export const MARKERS = ["base", "pivot", "compare", "swap", "place"] as const;

function trace(values: readonly number[]): Step[] {
  const r = new Recorder(values);
  const n = values.length;
  const ptrs = (pivot: number | null, i: number | null, j: number | null): Pointer[] => {
    const out: Pointer[] = [];
    if (pivot !== null) out.push({ label: "pivot", at: pivot, tone: "pivot" });
    if (i !== null) out.push({ label: "i", at: i, tone: "i" });
    if (j !== null) out.push({ label: "j", at: j, tone: "j" });
    return out;
  };
  const vars = (lo: number, hi: number, pivot: number | null, i: number | null, j: number | null) => ({
    lo,
    hi,
    pivot,
    i,
    j,
  });

  r.snap({
    range: [0, n - 1],
    narration: say("quick-sort.ready"),
    marker: null,
    vars: vars(0, n - 1, null, null, null),
  });

  const sort = (lo: number, hi: number) => {
    if (lo >= hi) {
      if (lo === hi) {
        r.settle(lo);
        r.snap({
          range: [lo, hi],
          narration: say("quick-sort.base", { lo }),
          marker: "base",
          vars: vars(lo, hi, null, null, null),
        });
      }
      return;
    }
    const pivot = r.value(hi);
    const range: [number, number] = [lo, hi];
    r.snap({
      roles: { [hi]: "pivot" },
      pointers: ptrs(hi, lo, null),
      range,
      narration: say("quick-sort.pivot", { lo, hi, pivot }),
      marker: "pivot",
      vars: vars(lo, hi, pivot, lo, null),
    });
    let i = lo;
    for (let j = lo; j < hi; j++) {
      r.comparisons++;
      const value = r.value(j);
      const smaller = value < pivot;
      r.snap({
        roles: { [hi]: "pivot", [j]: "compare" },
        pointers: ptrs(hi, i, j),
        range,
        narration: say(smaller ? "quick-sort.compare_smaller" : "quick-sort.compare_not", {
          value,
          pivot,
        }),
        marker: "compare",
        vars: vars(lo, hi, pivot, i, j),
      });
      if (smaller) {
        if (i !== j) {
          r.swap(i, j);
          r.moves++;
          r.writes += 2;
          r.snap({
            roles: { [hi]: "pivot", [i]: "swap", [j]: "swap" },
            pointers: ptrs(hi, i, j),
            range,
            narration: say("quick-sort.swap", { value, i }),
            marker: "swap",
            vars: vars(lo, hi, pivot, i, j),
          });
        } else {
          r.snap({
            roles: { [hi]: "pivot", [i]: "swap" },
            pointers: ptrs(hi, i, j),
            range,
            narration: say("quick-sort.swap_self", { value, i }),
            marker: "swap",
            vars: vars(lo, hi, pivot, i, j),
          });
        }
        i++;
      }
    }
    const moved = i !== hi;
    if (moved) {
      r.swap(i, hi);
      r.moves++;
      r.writes += 2;
    }
    r.settle(i);
    r.snap({
      roles: moved ? { [i]: "swap", [hi]: "swap" } : { [i]: "swap" },
      pointers: ptrs(null, i, null),
      range,
      narration: say("quick-sort.place", { pivot, i }),
      marker: "place",
      vars: vars(lo, hi, pivot, i, null),
    });
    sort(lo, i - 1);
    sort(i + 1, hi);
  };
  sort(0, n - 1);

  r.settleAll();
  r.snap({
    narration: say("quick-sort.done", {
      comparisons: r.comparisons,
      moves: r.moves,
      n,
    }),
    marker: null,
  });
  return r.steps;
}

export const quickSort: TracerDef = {
  id: "quick-sort",
  subject: "sorting",
  scene: "bars",
  markers: MARKERS,
  narrationKeys: KEYS,
  movesLabel: "swaps",
  legend: ["pivot", "compare", "swap", "sorted", "out"],
  vars: ["lo", "hi", "pivot", "i", "j"],
  trace,
};
