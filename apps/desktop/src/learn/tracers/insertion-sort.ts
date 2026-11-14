// Insertion sort. Mirrors content/learn/sorting/insertion-sort/sample.{py,ts} line for
// line (`@step` markers). The key is drawn lifted above the row; the slot it hovers over
// is the gap, and each shift slides a bigger value right into the gap.
import { Recorder } from "../engine/recorder";
import { SORT_INPUT } from "../engine/cases";
import { type NarrationKey, type NarrationParams, sortCounters, type Step, type TracerDef } from "../engine/types";

type Key = NarrationKey<"insertion-sort">;
const KEYS = [
  "insertion-sort.ready",
  "insertion-sort.take",
  "insertion-sort.compare_bigger",
  "insertion-sort.compare_stop",
  "insertion-sort.shift",
  "insertion-sort.insert",
  "insertion-sort.done",
] as const satisfies readonly Key[];
const say = (key: (typeof KEYS)[number], params?: NarrationParams<number>) => ({ key, params });

export const MARKERS = ["take", "compare", "shift", "insert"] as const;

function trace(values: readonly number[]): Step[] {
  const r = new Recorder(values);
  const n = values.length;
  r.snap({
    range: [0, 0],
    narration: say("insertion-sort.ready"),
    marker: null,
    vars: { i: null, j: null, key: null },
  });

  for (let i = 1; i < n; i++) {
    const key = r.value(i);
    const range: [number, number] = [0, i];
    let gap = i; // where the key's item sits in `items` while it is lifted
    r.snap({
      roles: { [gap]: "key" },
      lifted: gap,
      pointers: [{ label: "i", at: i, tone: "i" }],
      range,
      narration: say("insertion-sort.take", { key, i }),
      marker: "take",
      vars: { i, j: i - 1, key },
    });
    let j = i - 1;
    while (j >= 0) {
      r.comparisons++;
      const value = r.value(j);
      const bigger = value > key;
      r.snap({
        roles: { [j]: "compare", [gap]: "key" },
        lifted: gap,
        pointers: [
          { label: "i", at: i, tone: "i" },
          { label: "j", at: j, tone: "j" },
        ],
        range,
        narration: say(bigger ? "insertion-sort.compare_bigger" : "insertion-sort.compare_stop", {
          value,
          key,
        }),
        marker: "compare",
        vars: { i, j, key },
      });
      if (!bigger) break;
      // a[j + 1] = a[j]: the bigger value slides right into the gap; the gap moves left.
      r.swap(j, gap);
      r.moves++;
      r.writes++;
      gap = j;
      r.snap({
        roles: { [j + 1]: "swap", [gap]: "key" },
        lifted: gap,
        pointers: [
          { label: "i", at: i, tone: "i" },
          { label: "j", at: j, tone: "j" },
        ],
        range,
        narration: say("insertion-sort.shift", { value, to: j + 1 }),
        marker: "shift",
        vars: { i, j, key },
      });
      j--;
    }
    r.writes++;
    r.snap({
      roles: { [gap]: "swap" },
      pointers: [{ label: "i", at: i, tone: "i" }],
      range,
      narration: say("insertion-sort.insert", { key, at: gap }),
      marker: "insert",
      vars: { i, j, key },
    });
  }

  r.settleAll();
  r.snap({
    narration: say("insertion-sort.done", { comparisons: r.comparisons, moves: r.moves, n }),
    marker: null,
  });
  return r.steps;
}

export const insertionSort: TracerDef = {
  id: "insertion-sort",
  subject: "sorting",
  compareGroup: "sort",
  scene: "array",
  lifts: true,
  markers: MARKERS,
  narrationKeys: KEYS,
  limits: SORT_INPUT,
  input: { kind: "values" },
  counters: sortCounters("shifts"),
  roleLabels: { swap: "shift" },
  legend: ["key", "compare", "swap", "sorted", "out"],
  vars: ["i", "j", "key"],
  trace,
};
