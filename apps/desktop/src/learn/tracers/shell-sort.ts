// Shell sort with Shell's gaps n/2, n/4, …, 1. Mirrors
// content/learn/sorting/shell-sort/sample.{py,ts}: each pass is an insertion sort over the
// values `gap` apart, so a value can jump many places at once; the last pass (gap 1) is a
// plain insertion sort over an almost sorted array.
import { SORT_INPUT } from "../engine/cases";
import { Recorder } from "../engine/recorder";
import { type NarrationKey, type NarrationParams, type Pointer, sortCounters, type Step, type TracerDef } from "../engine/types";

type Key = NarrationKey<"shell-sort">;
const KEYS = [
  "shell-sort.ready",
  "shell-sort.gap",
  "shell-sort.take",
  "shell-sort.compare_bigger",
  "shell-sort.compare_stop",
  "shell-sort.shift",
  "shell-sort.insert",
  "shell-sort.done",
] as const satisfies readonly Key[];
const say = (key: (typeof KEYS)[number], params?: NarrationParams<number>) => ({ key, params });

export const MARKERS = ["gap", "take", "compare", "shift", "insert"] as const;

function trace(values: readonly number[]): Step[] {
  const r = new Recorder(values);
  const n = values.length;
  const ptrs = (i: number, j: number | null): Pointer[] => [
    { label: "i", at: i, tone: "i" },
    ...(j === null ? [] : [{ label: "j", at: j, tone: "j" } as Pointer]),
  ];
  r.snap({ narration: say("shell-sort.ready", { n }), marker: null, vars: { gap: null, i: null, j: null, key: null } });

  for (let gap = Math.floor(n / 2); gap > 0; gap = Math.floor(gap / 2)) {
    r.snap({ narration: say("shell-sort.gap", { gap }), marker: "gap", vars: { gap, i: null, j: null, key: null } });
    for (let i = gap; i < n; i++) {
      const key = r.value(i);
      let hole = i; // where the key's item sits while it is lifted
      r.snap({
        roles: { [hole]: "key" },
        lifted: hole,
        pointers: ptrs(i, null),
        narration: say("shell-sort.take", { key, i }),
        marker: "take",
        vars: { gap, i, j: i, key },
      });
      let j = i;
      while (j >= gap) {
        r.comparisons++;
        const value = r.value(j - gap);
        const bigger = value > key;
        r.snap({
          roles: { [j - gap]: "compare", [hole]: "key" },
          lifted: hole,
          pointers: ptrs(i, j - gap),
          narration: say(bigger ? "shell-sort.compare_bigger" : "shell-sort.compare_stop", { value, key, gap }),
          marker: "compare",
          vars: { gap, i, j, key },
        });
        if (!bigger) break;
        // a[j] = a[j - gap]: the bigger value jumps `gap` places right; the hole moves left.
        r.swap(j - gap, hole);
        r.moves++;
        r.writes++;
        hole = j - gap;
        r.snap({
          roles: { [j]: "swap", [hole]: "key" },
          lifted: hole,
          pointers: ptrs(i, j),
          narration: say("shell-sort.shift", { value, from: j - gap, to: j }),
          marker: "shift",
          vars: { gap, i, j, key },
        });
        j -= gap;
      }
      r.writes++;
      r.snap({
        roles: { [hole]: "swap" },
        pointers: ptrs(i, null),
        narration: say("shell-sort.insert", { key, at: hole }),
        marker: "insert",
        vars: { gap, i, j, key },
      });
    }
  }
  r.settleAll();
  r.snap({ narration: say("shell-sort.done", { comparisons: r.comparisons, moves: r.moves, n }), marker: null });
  return r.steps;
}

export const shellSort: TracerDef = {
  id: "shell-sort",
  subject: "sorting",
  compareGroup: "sort",
  scene: "array",
  lifts: true,
  limits: SORT_INPUT,
  input: { kind: "values" },
  markers: MARKERS,
  narrationKeys: KEYS,
  counters: sortCounters("shifts"),
  legend: ["key", "compare", "swap", "sorted"],
  roleLabels: { swap: "shift" },
  vars: ["gap", "i", "j", "key"],
  trace,
};
