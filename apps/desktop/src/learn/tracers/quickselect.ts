// Quickselect: the k-th smallest value without sorting everything. Mirrors
// content/learn/sorting/quickselect/sample.{py,ts}: the same Lomuto partition as Quick
// Sort puts one pivot in its final place; if that place is k - 1 we are done, otherwise
// only the side holding position k - 1 is searched again.
import { Recorder } from "../engine/recorder";
import type { NarrationKey, NarrationParams, Pointer, Step, TracerDef } from "../engine/types";

type Key = NarrationKey<"quickselect">;
const KEYS = [
  "quickselect.ready",
  "quickselect.pivot",
  "quickselect.compare_smaller",
  "quickselect.compare_not",
  "quickselect.swap",
  "quickselect.swap_self",
  "quickselect.place",
  "quickselect.found",
  "quickselect.go_right",
  "quickselect.go_left",
  "quickselect.done",
] as const satisfies readonly Key[];
const say = (key: (typeof KEYS)[number], params?: NarrationParams<number>) => ({ key, params });

export const MARKERS = ["pivot", "compare", "swap", "place", "found", "side"] as const;

function trace(input: readonly number[]): Step[] {
  const k = input[0]!;
  const values = input.slice(1);
  const r = new Recorder(values);
  const n = values.length;
  const want = k - 1;
  const target: Pointer = { label: "k−1", at: want, tone: "key" };
  const ptrs = (pivot: number | null, i: number | null, j: number | null): Pointer[] => [
    ...(pivot === null ? [] : [{ label: "pivot", at: pivot, tone: "pivot" } as Pointer]),
    ...(i === null ? [] : [{ label: "i", at: i, tone: "i" } as Pointer]),
    ...(j === null ? [] : [{ label: "j", at: j, tone: "j" } as Pointer]),
    target,
  ];
  let lo = 0;
  let hi = n - 1;
  const vars = (pivot: number | null, i: number | null, j: number | null) => ({ k, lo, hi, pivot, i, j });
  r.snap({ range: [lo, hi], pointers: ptrs(null, null, null), narration: say("quickselect.ready", { k, n }), marker: null, vars: vars(null, null, null) });
  let answer = 0;
  for (;;) {
    const range: [number, number] = [lo, hi];
    const pivot = r.value(hi);
    r.snap({ roles: { [hi]: "pivot" }, pointers: ptrs(hi, lo, null), range, narration: say("quickselect.pivot", { lo, hi, pivot }), marker: "pivot", vars: vars(pivot, lo, null) });
    let i = lo;
    for (let j = lo; j < hi; j++) {
      r.comparisons++;
      const value = r.value(j);
      const smaller = value < pivot;
      r.snap({
        roles: { [hi]: "pivot", [j]: "compare" },
        pointers: ptrs(hi, i, j),
        range,
        narration: say(smaller ? "quickselect.compare_smaller" : "quickselect.compare_not", { value, pivot }),
        marker: "compare",
        vars: vars(pivot, i, j),
      });
      if (smaller) {
        if (i !== j) {
          r.swap(i, j);
          r.moves++;
          r.writes += 2;
        }
        r.snap({
          roles: i !== j ? { [hi]: "pivot", [i]: "swap", [j]: "swap" } : { [hi]: "pivot", [i]: "swap" },
          pointers: ptrs(hi, i, j),
          range,
          narration: say(i !== j ? "quickselect.swap" : "quickselect.swap_self", { value, i }),
          marker: "swap",
          vars: vars(pivot, i, j),
        });
        i++;
      }
    }
    if (i !== hi) {
      r.swap(i, hi);
      r.moves++;
      r.writes += 2;
    }
    r.settle(i);
    r.snap({ roles: { [i]: "swap" }, pointers: ptrs(null, i, null), range, narration: say("quickselect.place", { pivot, i }), marker: "place", vars: vars(pivot, i, null) });
    if (i === want) {
      answer = r.value(i);
      r.snap({ roles: { [i]: "key" }, pointers: ptrs(null, i, null), range, narration: say("quickselect.found", { p: i, k, value: answer }), marker: "found", vars: vars(pivot, i, null) });
      break;
    }
    const right = i < want;
    r.snap({
      roles: { [i]: "sorted" },
      pointers: ptrs(null, i, null),
      range,
      narration: say(right ? "quickselect.go_right" : "quickselect.go_left", { p: i, want }),
      marker: "side",
      vars: vars(pivot, i, null),
    });
    if (right) lo = i + 1;
    else hi = i - 1;
  }
  r.snap({
    roles: { [want]: "key" },
    pointers: [target],
    narration: say("quickselect.done", { k, value: answer, comparisons: r.comparisons }),
    marker: null,
    vars: vars(null, null, null),
    result: [answer],
  });
  return r.steps;
}

export const quickselect: TracerDef = {
  id: "quickselect",
  subject: "sorting",
  scene: "array",
  limits: { min_len: 2, max_len: 17, min: 1, max: 99 },
  input: {
    kind: "target",
    targetLabel: "k",
    pickTarget: (values, next) => 1 + Math.floor(next() * values.length),
    checkTarget: (k, values) => (k >= 1 && k <= values.length ? null : { min: 1, max: values.length }),
  },
  markers: MARKERS,
  narrationKeys: KEYS,
  counters: [
    { slot: "comparisons", label: "comparisons" },
    { slot: "moves", label: "swaps" },
  ],
  legend: ["pivot", "compare", "swap", "key", "sorted", "out"],
  roleLabels: { key: "answer" },
  vars: ["k", "lo", "hi", "pivot", "i", "j"],
  trace,
};
