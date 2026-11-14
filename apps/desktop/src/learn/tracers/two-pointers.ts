// Two pointers: a pair with a given sum in sorted values. Mirrors
// content/learn/searching/two-pointers/sample.{py,ts}: start at both ends; a sum too small
// can only grow by moving i right, a sum too big only shrink by moving j left, so every
// step rules out one value for good.
import { Recorder } from "../engine/recorder";
import type { NarrationKey, NarrationParams, Pointer, Role, Step, TracerDef } from "../engine/types";

type Key = NarrationKey<"two-pointers">;
const KEYS = [
  "two-pointers.ready",
  "two-pointers.sum",
  "two-pointers.found",
  "two-pointers.move_i",
  "two-pointers.move_j",
  "two-pointers.missing",
] as const satisfies readonly Key[];
const say = (key: (typeof KEYS)[number], params?: NarrationParams<number>) => ({ key, params });

export const MARKERS = ["sum", "found", "move", "missing"] as const;
export const PAIR_INPUT = { min_len: 3, max_len: 17, min: 1, max: 60 };

function trace(input: readonly number[]): Step[] {
  const target = input[0]!;
  const r = new Recorder(input.slice(1));
  const n = r.items.length;
  let i = 0;
  let j = n - 1;
  const out = (): Record<number, Role> => {
    const roles: Record<number, Role> = {};
    for (let p = 0; p < n; p++) if (p < i || p > j) roles[p] = "out";
    return roles;
  };
  const ptrs = (): Pointer[] => [
    { label: "i", at: i, tone: "i" },
    { label: "j", at: j, tone: "j" },
  ];
  r.snap({ pointers: ptrs(), narration: say("two-pointers.ready", { target, n }), marker: null, vars: { target, i, j, total: null } });
  while (i < j) {
    const total = r.value(i) + r.value(j);
    r.comparisons++;
    r.snap({ roles: { ...out(), [i]: "compare", [j]: "compare" }, pointers: ptrs(), narration: say("two-pointers.sum", { a: r.value(i), b: r.value(j), total, target }), marker: "sum", vars: { target, i, j, total } });
    if (total === target) {
      r.snap({ roles: { ...out(), [i]: "sorted", [j]: "sorted" }, pointers: ptrs(), narration: say("two-pointers.found", { i, j, total }), marker: "found", vars: { target, i, j, total }, result: [i, j] });
      return r.steps;
    }
    const small = total < target;
    if (small) i++;
    else j--;
    r.snap({
      roles: { ...out(), [small ? i - 1 : j + 1]: "out" },
      pointers: ptrs(),
      narration: say(small ? "two-pointers.move_i" : "two-pointers.move_j", { total, target, value: small ? r.value(i - 1) : r.value(j + 1) }),
      marker: "move",
      vars: { target, i, j, total },
    });
  }
  r.snap({ roles: out(), pointers: ptrs(), narration: say("two-pointers.missing", { target, checks: r.comparisons }), marker: "missing", vars: { target, i, j, total: null }, result: [-1, -1] });
  return r.steps;
}

export const twoPointers: TracerDef = {
  id: "two-pointers",
  subject: "searching",
  scene: "array",
  limits: PAIR_INPUT,
  input: {
    kind: "sorted-target",
    // Usually the sum of a real pair (when it fits the limit), sometimes one that is missing.
    pickTarget: (values, next) => {
      const a = Math.floor(next() * values.length);
      let b = Math.floor(next() * values.length);
      if (b === a) b = (a + 1) % values.length;
      const sum = values[a]! + values[b]!;
      if (next() < 0.75 && sum <= 60) return sum;
      return Math.min(60, values[0]! + values[1]! + (next() < 0.5 ? 1 : 0));
    },
  },
  markers: MARKERS,
  narrationKeys: KEYS,
  counters: [{ slot: "comparisons", label: "sums" }],
  legend: ["compare", "sorted", "out"],
  roleLabels: { sorted: "found", out: "ruled_out" },
  vars: ["target", "i", "j", "total"],
  trace,
};
