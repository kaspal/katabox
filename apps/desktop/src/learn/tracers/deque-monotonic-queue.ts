// Deque as a monotonic queue: the maximum of every window of k values. Mirrors
// content/learn/linear/deque-monotonic-queue/sample.{py,ts}: the deque holds positions
// whose values decrease from front to back; a new value pops every smaller-or-equal one
// off the back (they can never be a maximum again), the front drops out when it leaves the
// window, and the front is always the window's maximum. Each position enters and leaves
// once: O(n).
import { Recorder } from "../engine/recorder";
import type { Item, NarrationKey, NarrationParams, Role, Step, TracerDef } from "../engine/types";

type Key = NarrationKey<"deque-monotonic-queue">;
const KEYS = [
  "deque-monotonic-queue.ready",
  "deque-monotonic-queue.pop_back",
  "deque-monotonic-queue.push_back",
  "deque-monotonic-queue.pop_front",
  "deque-monotonic-queue.emit",
  "deque-monotonic-queue.done",
] as const satisfies readonly Key[];
const say = (key: (typeof KEYS)[number], params?: NarrationParams<number>) => ({ key, params });

export const MARKERS = ["pop-back", "push-back", "pop-front", "emit"] as const;

function trace(input: readonly number[]): Step[] {
  const k = input[0]!;
  const r = new Recorder(input.slice(1));
  const n = r.items.length;
  const dq: number[] = [];
  const out: number[] = [];
  const snap = (narration: ReturnType<typeof say>, marker: string | null, i: number, roles: Record<number, Role> = {}) => {
    const lo = Math.max(0, i - k + 1);
    r.snap({
      narration,
      marker,
      roles,
      window: i >= 0 ? [lo, i] : null,
      pointers: i >= 0 ? [{ label: "i", at: i, tone: "i" }] : [],
      aux: {
        label: "deque",
        cells: dq.map((p): Item => ({ id: p, value: r.value(p) })),
        roles: dq.length ? { 0: "pivot" } : {},
      },
      vars: { k, i: i >= 0 ? i : null, front: dq.length ? dq[0]! : null, max: out.length ? out[out.length - 1]! : null },
    });
  };
  snap(say("deque-monotonic-queue.ready", { k, n }), null, -1);
  for (let i = 0; i < n; i++) {
    const v = r.value(i);
    while (dq.length > 0 && r.value(dq[dq.length - 1]!) <= v) {
      const p = dq.pop()!;
      r.comparisons++;
      r.moves++;
      snap(say("deque-monotonic-queue.pop_back", { value: r.value(p), at: p, by: v }), "pop-back", i, { [p]: "out", [i]: "compare" });
    }
    if (dq.length > 0) r.comparisons++;
    dq.push(i);
    snap(say("deque-monotonic-queue.push_back", { value: v, at: i }), "push-back", i, { [i]: "swap" });
    if (dq[0]! <= i - k) {
      const p = dq.shift()!;
      r.moves++;
      snap(say("deque-monotonic-queue.pop_front", { at: p, value: r.value(p) }), "pop-front", i, { [p]: "out" });
    }
    if (i >= k - 1) {
      const m = r.value(dq[0]!);
      out.push(m);
      snap(say("deque-monotonic-queue.emit", { lo: i - k + 1, hi: i, max: m }), "emit", i, { [dq[0]!]: "pivot" });
    }
  }
  r.snap({ narration: say("deque-monotonic-queue.done", { windows: out.length, pops: r.moves }), marker: null, vars: { k, i: null, front: null, max: null }, result: out });
  return r.steps;
}

export const dequeMonotonicQueue: TracerDef = {
  id: "deque-monotonic-queue",
  subject: "linear",
  scene: "array",
  limits: { min_len: 2, max_len: 13, min: 1, max: 99 },
  input: {
    kind: "target",
    targetLabel: "k",
    pickTarget: (values, next) => Math.min(values.length, 2 + Math.floor(next() * 3)),
    checkTarget: (k, values) => (k >= 1 && k <= values.length ? null : { min: 1, max: values.length }),
  },
  markers: MARKERS,
  narrationKeys: KEYS,
  counters: [
    { slot: "comparisons", label: "comparisons" },
    { slot: "moves", label: "pops" },
  ],
  legend: ["swap", "compare", "pivot", "out"],
  roleLabels: { swap: "entering", compare: "current", pivot: "window_max", out: "dropped" },
  vars: ["k", "i", "front", "max"],
  trace,
};
