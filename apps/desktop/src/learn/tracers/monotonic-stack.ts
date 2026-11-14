// Monotonic stack: the next greater value for every position. Mirrors
// content/learn/searching/monotonic-stack/sample.{py,ts}: the stack holds the positions
// still waiting for a bigger value, their values decreasing from bottom to top. A new
// value pops (and answers) every smaller one on top, then waits itself. Each position is
// pushed and popped at most once: linear time.
import { Recorder } from "../engine/recorder";
import type { Item, NarrationKey, Role, StackEntry, Step, TracerDef } from "../engine/types";

type Key = NarrationKey<"monotonic-stack">;
const KEYS = [
  "monotonic-stack.ready",
  "monotonic-stack.compare_pop",
  "monotonic-stack.compare_stop",
  "monotonic-stack.pop",
  "monotonic-stack.push",
  "monotonic-stack.done",
] as const satisfies readonly Key[];
const say = (key: (typeof KEYS)[number], params?: Record<string, number>) => ({ key, params });

export const MARKERS = ["compare", "pop", "push"] as const;

function trace(values: readonly number[]): Step[] {
  const r = new Recorder(values);
  const n = values.length;
  const result: number[] = new Array<number>(n).fill(-1);
  const answered: (Item | null)[] = new Array<Item | null>(n).fill(null);
  const stack: number[] = [];
  const frame = (roles: Record<number, Role>, auxRoles: Record<number, Role> = {}) => ({
    roles,
    aux: { label: "answer" as const, cells: answered.map((c) => (c ? { ...c } : null)), roles: auxRoles },
    stack: stack.map((p, k): StackEntry => ({ id: p, text: `[${p}] ${r.value(p)}`, role: k === stack.length - 1 ? "key" : "idle" })),
  });
  const vars = (i: number | null) => ({ i, top: stack.length ? stack[stack.length - 1]! : null, size: stack.length });
  r.snap({ ...frame({}), narration: say("monotonic-stack.ready", { n }), marker: null, vars: vars(null) });
  for (let i = 0; i < n; i++) {
    const value = r.value(i);
    while (stack.length > 0) {
      const top = stack[stack.length - 1]!;
      r.comparisons++;
      const smaller = r.value(top) < value;
      r.snap({
        ...frame({ [top]: "compare", [i]: "pivot" }),
        pointers: [{ label: "i", at: i, tone: "i" }],
        narration: say(smaller ? "monotonic-stack.compare_pop" : "monotonic-stack.compare_stop", { top: r.value(top), value, at: top }),
        marker: "compare",
        vars: vars(i),
      });
      if (!smaller) break;
      stack.pop();
      result[top] = value;
      answered[top] = { id: 1000 + top, value };
      r.moves++;
      r.snap({
        ...frame({ [top]: "sorted", [i]: "pivot" }, { [top]: "swap" }),
        pointers: [{ label: "i", at: i, tone: "i" }],
        narration: say("monotonic-stack.pop", { at: top, value: r.value(top), answer: value }),
        marker: "pop",
        vars: vars(i),
      });
    }
    stack.push(i);
    r.writes++;
    r.snap({
      ...frame({ [i]: "key" }),
      pointers: [{ label: "i", at: i, tone: "i" }],
      narration: say("monotonic-stack.push", { at: i, value, size: stack.length }),
      marker: "push",
      vars: vars(i),
    });
  }
  r.snap({
    ...frame({}),
    narration: say("monotonic-stack.done", { n, left: stack.length, pops: r.moves }),
    marker: null,
    vars: vars(null),
    result,
  });
  const deepest = Math.max(1, ...r.steps.map((s) => s.stack?.length ?? 0));
  for (const s of r.steps) s.stackDepth = deepest;
  return r.steps;
}

export const monotonicStack: TracerDef = {
  id: "monotonic-stack",
  subject: "searching",
  scene: "monostack",
  limits: { min_len: 1, max_len: 12, min: 1, max: 99 },
  input: { kind: "values" },
  markers: MARKERS,
  narrationKeys: KEYS,
  counters: [
    { slot: "comparisons", label: "comparisons" },
    { slot: "writes", label: "pushes" },
    { slot: "moves", label: "pops" },
  ],
  legend: ["pivot", "compare", "key", "sorted"],
  roleLabels: { pivot: "current", compare: "stack_top", key: "waiting", sorted: "answered" },
  vars: ["i", "top", "size"],
  trace,
};
