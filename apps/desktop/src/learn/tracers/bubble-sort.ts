// Bubble sort with the early exit. Mirrors content/learn/sorting/bubble-sort/sample.{py,ts}
// line for line (`@step` markers).
import { Recorder } from "../engine/recorder";
import { SORT_INPUT } from "../engine/cases";
import { sortCounters, type NarrationKey, type Step, type TracerDef } from "../engine/types";

type Key = NarrationKey<"bubble-sort">;
const KEYS = [
  "bubble-sort.ready",
  "bubble-sort.pass",
  "bubble-sort.compare_bigger",
  "bubble-sort.compare_ok",
  "bubble-sort.swap",
  "bubble-sort.settled",
  "bubble-sort.no_swaps",
  "bubble-sort.done",
] as const satisfies readonly Key[];
const say = (key: (typeof KEYS)[number], params?: Record<string, number>) => ({ key, params });

export const MARKERS = ["pass", "compare", "swap", "check"] as const;

function trace(values: readonly number[]): Step[] {
  const r = new Recorder(values);
  const n = values.length;
  r.snap({ narration: say("bubble-sort.ready"), marker: null, vars: { end: null, j: null, swapped: null } });

  for (let end = n - 1; end > 0; end--) {
    const range: [number, number] = [0, end];
    r.snap({
      range,
      narration: say("bubble-sort.pass", { end }),
      marker: "pass",
      vars: { end, j: null, swapped: false },
    });
    let swapped = false;
    for (let j = 0; j < end; j++) {
      r.comparisons++;
      const a = r.value(j);
      const b = r.value(j + 1);
      const bigger = a > b;
      r.snap({
        roles: { [j]: "compare", [j + 1]: "compare" },
        pointers: [{ label: "j", at: j, tone: "j" }],
        range,
        narration: say(bigger ? "bubble-sort.compare_bigger" : "bubble-sort.compare_ok", { a, b }),
        marker: "compare",
        vars: { end, j, swapped },
      });
      if (bigger) {
        r.swap(j, j + 1);
        r.moves++;
        r.writes += 2;
        swapped = true;
        r.snap({
          roles: { [j]: "swap", [j + 1]: "swap" },
          pointers: [{ label: "j", at: j, tone: "j" }],
          range,
          narration: say("bubble-sort.swap", { a, b }),
          marker: "swap",
          vars: { end, j, swapped },
        });
      }
    }
    r.settle(end);
    r.snap({
      range,
      narration: swapped
        ? say("bubble-sort.settled", { value: r.value(end), end })
        : say("bubble-sort.no_swaps"),
      marker: "check",
      vars: { end, j: null, swapped },
    });
    if (!swapped) break;
  }

  r.settleAll();
  r.snap({
    narration: say("bubble-sort.done", { comparisons: r.comparisons, moves: r.moves, n }),
    marker: null,
  });
  return r.steps;
}

export const bubbleSort: TracerDef = {
  id: "bubble-sort",
  subject: "sorting",
  scene: "bars",
  markers: MARKERS,
  narrationKeys: KEYS,
  limits: SORT_INPUT,
  input: { kind: "values" },
  counters: sortCounters("swaps"),
  legend: ["compare", "swap", "sorted", "out"],
  vars: ["end", "j", "swapped"],
  trace,
};
