// Sliding window: the longest run of values whose sum stays within a limit (all values
// positive). Mirrors content/learn/searching/sliding-window/sample.{py,ts}: the right edge
// moves one step at a time; whenever the sum goes over, the left edge moves until it fits
// again. Each edge only moves forward, so the work is linear.
import { Recorder } from "../engine/recorder";
import type { NarrationKey, NarrationParams, Pointer, Step, TracerDef } from "../engine/types";

type Key = NarrationKey<"sliding-window">;
const KEYS = [
  "sliding-window.ready",
  "sliding-window.grow",
  "sliding-window.shrink",
  "sliding-window.best",
  "sliding-window.not_best",
  "sliding-window.done",
] as const satisfies readonly Key[];
const say = (key: (typeof KEYS)[number], params?: NarrationParams<number>) => ({ key, params });

export const MARKERS = ["grow", "shrink", "check"] as const;

function trace(input: readonly number[]): Step[] {
  const limit = input[0]!;
  const r = new Recorder(input.slice(1));
  const n = r.items.length;
  let best = 0;
  let bestStart = 0;
  let total = 0;
  let lo = 0;
  const ptrs = (hi: number): Pointer[] => [
    { label: "lo", at: Math.min(lo, n - 1), tone: "i" },
    { label: "hi", at: hi, tone: "j" },
  ];
  const vars = (hi: number | null) => ({ limit, lo, hi, total, best });
  r.snap({ narration: say("sliding-window.ready", { limit, n }), marker: null, vars: vars(null) });
  for (let hi = 0; hi < n; hi++) {
    total += r.value(hi);
    r.moves++;
    r.snap({
      roles: { [hi]: "swap" },
      window: lo <= hi ? [lo, hi] : null,
      pointers: ptrs(hi),
      narration: say("sliding-window.grow", { value: r.value(hi), total, limit }),
      marker: "grow",
      vars: vars(hi),
    });
    while (total > limit) {
      const value = r.value(lo);
      total -= value;
      lo++;
      r.moves++;
      r.snap({
        roles: { [lo - 1]: "out" },
        window: lo <= hi ? [lo, hi] : null,
        pointers: ptrs(hi),
        narration: say("sliding-window.shrink", { value, total, limit }),
        marker: "shrink",
        vars: vars(hi),
      });
    }
    r.comparisons++;
    const length = hi - lo + 1;
    const better = length > best;
    if (better) {
      best = length;
      bestStart = lo;
    }
    r.snap({
      window: lo <= hi ? [lo, hi] : null,
      pointers: ptrs(hi),
      narration: say(better ? "sliding-window.best" : "sliding-window.not_best", { length, best, lo, hi }),
      marker: "check",
      vars: vars(hi),
    });
  }
  r.snap({
    window: best > 0 ? [bestStart, bestStart + best - 1] : null,
    narration: say("sliding-window.done", { best, start: bestStart, moves: r.moves, n }),
    marker: null,
    vars: { limit, lo: bestStart, hi: best > 0 ? bestStart + best - 1 : null, total: null, best },
    result: [best, bestStart],
  });
  return r.steps;
}

export const slidingWindow: TracerDef = {
  id: "sliding-window",
  subject: "searching",
  scene: "array",
  limits: { min_len: 2, max_len: 17, min: 1, max: 40 },
  input: {
    kind: "target",
    targetLabel: "limit",
    pickTarget: (values, next) => {
      const avg = values.reduce((s, v) => s + v, 0) / values.length;
      return Math.max(1, Math.min(40, Math.round(avg * (2 + next() * 2))));
    },
  },
  markers: MARKERS,
  narrationKeys: KEYS,
  counters: [{ slot: "moves", label: "edge_moves" }],
  legend: ["swap", "out"],
  roleLabels: { swap: "entering", out: "leaving" },
  vars: ["limit", "lo", "hi", "total", "best"],
  trace,
};
