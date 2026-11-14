// Selection sort. Mirrors content/learn/sorting/selection-sort/sample.{py,ts}: each pass
// scans the unsorted part for its smallest value and swaps it to the front of that part.
import { SORT_INPUT } from "../engine/cases";
import { Recorder } from "../engine/recorder";
import { sortCounters, type NarrationKey, type Pointer, type Step, type TracerDef } from "../engine/types";

type Key = NarrationKey<"selection-sort">;
const KEYS = [
  "selection-sort.ready",
  "selection-sort.start",
  "selection-sort.compare_smaller",
  "selection-sort.compare_not",
  "selection-sort.new_min",
  "selection-sort.swap",
  "selection-sort.stay",
  "selection-sort.done",
] as const satisfies readonly Key[];
const say = (key: (typeof KEYS)[number], params?: Record<string, number>) => ({ key, params });

export const MARKERS = ["start", "compare", "new-min", "swap"] as const;

function trace(values: readonly number[]): Step[] {
  const r = new Recorder(values);
  const n = values.length;
  const ptrs = (i: number, j: number | null, min: number): Pointer[] => [
    { label: "i", at: i, tone: "i" },
    ...(j === null ? [] : [{ label: "j", at: j, tone: "j" } as Pointer]),
    { label: "min", at: min, tone: "pivot" },
  ];
  r.snap({ range: [0, n - 1], narration: say("selection-sort.ready"), marker: null, vars: { i: null, j: null, smallest: null } });

  for (let i = 0; i < n - 1; i++) {
    let smallest = i;
    const range: [number, number] = [i, n - 1];
    r.snap({
      roles: { [i]: "pivot" },
      pointers: ptrs(i, null, smallest),
      range,
      narration: say("selection-sort.start", { i, value: r.value(i) }),
      marker: "start",
      vars: { i, j: null, smallest },
    });
    for (let j = i + 1; j < n; j++) {
      r.comparisons++;
      const smaller = r.value(j) < r.value(smallest);
      r.snap({
        roles: { [smallest]: "pivot", [j]: "compare" },
        pointers: ptrs(i, j, smallest),
        range,
        narration: say(smaller ? "selection-sort.compare_smaller" : "selection-sort.compare_not", {
          value: r.value(j),
          min: r.value(smallest),
        }),
        marker: "compare",
        vars: { i, j, smallest },
      });
      if (smaller) {
        smallest = j;
        r.snap({
          roles: { [smallest]: "pivot" },
          pointers: ptrs(i, j, smallest),
          range,
          narration: say("selection-sort.new_min", { value: r.value(j), at: j }),
          marker: "new-min",
          vars: { i, j, smallest },
        });
      }
    }
    const value = r.value(smallest);
    if (smallest !== i) {
      r.swap(i, smallest);
      r.moves++;
      r.writes += 2;
    }
    r.settle(i);
    r.snap({
      roles: smallest !== i ? { [i]: "swap", [smallest]: "swap" } : { [i]: "swap" },
      pointers: ptrs(i, null, smallest),
      range,
      narration: say(smallest !== i ? "selection-sort.swap" : "selection-sort.stay", { value, i }),
      marker: "swap",
      vars: { i, j: null, smallest },
    });
  }
  r.settleAll();
  r.snap({ narration: say("selection-sort.done", { comparisons: r.comparisons, moves: r.moves, n }), marker: null });
  return r.steps;
}

export const selectionSort: TracerDef = {
  id: "selection-sort",
  subject: "sorting",
  compareGroup: "sort",
  scene: "bars",
  limits: SORT_INPUT,
  input: { kind: "values" },
  markers: MARKERS,
  narrationKeys: KEYS,
  counters: sortCounters("swaps"),
  legend: ["pivot", "compare", "swap", "sorted", "out"],
  roleLabels: { pivot: "minimum" },
  vars: ["i", "j", "smallest"],
  trace,
};
