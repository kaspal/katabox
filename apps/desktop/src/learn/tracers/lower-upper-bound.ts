// Lower and upper bound. Mirrors content/learn/searching/lower-upper-bound/sample.{py,ts}:
// two binary searches over a half-open range [lo, hi): the first position whose value is
// at least the target, then the first whose value is bigger. Between them sit exactly the
// copies of the target (none when they are equal).
import { pickSearchTarget, SEARCH_INPUT } from "../engine/search";
import { Recorder } from "../engine/recorder";
import type { NarrationKey, NarrationParams, Pointer, Role, Step, TracerDef } from "../engine/types";

type Key = NarrationKey<"lower-upper-bound">;
const KEYS = [
  "lower-upper-bound.ready",
  "lower-upper-bound.lower_right",
  "lower-upper-bound.lower_left",
  "lower-upper-bound.lower_done",
  "lower-upper-bound.upper_right",
  "lower-upper-bound.upper_left",
  "lower-upper-bound.upper_done",
  "lower-upper-bound.done",
] as const satisfies readonly Key[];
const say = (key: (typeof KEYS)[number], params?: NarrationParams<number>) => ({ key, params });

export const MARKERS = ["lower", "lower-done", "upper", "upper-done"] as const;

function trace(input: readonly number[]): Step[] {
  const target = input[0]!;
  const r = new Recorder(input.slice(1));
  const n = r.items.length;
  // Half-open [lo, hi): the pointer may sit one past the end, so show it at n − 1 at most.
  const ptrs = (lo: number, hi: number, mid: number | null): Pointer[] => [
    { label: "lo", at: Math.min(lo, n - 1), tone: "i" },
    ...(mid === null ? [] : [{ label: "mid", at: mid, tone: "pivot" } as Pointer]),
    { label: "hi", at: Math.min(hi, n - 1), tone: "j" },
  ];
  const vars = (lo: number, hi: number, mid: number | null, lower: number | null) => ({ target, lo, hi, mid, lower });
  r.snap({ narration: say("lower-upper-bound.ready", { target, n }), marker: null, vars: vars(0, n, null, null) });

  const search = (strict: boolean, lower: number | null, keep: Record<number, Role>): number => {
    let lo = 0;
    let hi = n;
    while (lo < hi) {
      const mid = Math.floor((lo + hi) / 2);
      const value = r.value(mid);
      r.comparisons++;
      const right = strict ? value <= target : value < target;
      const before = { lo, hi };
      if (right) lo = mid + 1;
      else hi = mid;
      const which = strict ? "upper" : "lower";
      r.snap({
        roles: { ...keep, [mid]: "pivot" },
        pointers: ptrs(before.lo, before.hi, mid),
        range: before.lo < before.hi ? [before.lo, before.hi - 1] : null,
        narration: say(`lower-upper-bound.${which}_${right ? "right" : "left"}`, { value, target, mid, lo, hi }),
        marker: which,
        vars: vars(lo, hi, mid, lower),
      });
    }
    return lo;
  };

  const lower = search(false, null, {});
  const lowerRoles: Record<number, Role> = lower < n ? { [lower]: "key" } : {};
  r.snap({
    roles: lowerRoles,
    pointers: [{ label: "lower", at: Math.min(lower, n - 1), tone: "key" }],
    narration: say("lower-upper-bound.lower_done", { lower, target, n }),
    marker: "lower-done",
    vars: vars(lower, lower, null, lower),
  });
  const upper = search(true, lower, lowerRoles);
  const roles: Record<number, Role> = {};
  for (let p = lower; p < upper; p++) roles[p] = "sorted";
  r.snap({
    roles,
    pointers: [
      { label: "lower", at: Math.min(lower, n - 1), tone: "key" },
      { label: "upper", at: Math.min(upper, n - 1), tone: "j" },
    ],
    narration: say("lower-upper-bound.upper_done", { upper, target, n }),
    marker: "upper-done",
    vars: vars(upper, upper, null, lower),
  });
  r.snap({
    roles,
    narration: say("lower-upper-bound.done", { lower, upper, num: upper - lower, target }),
    marker: null,
    vars: vars(upper, upper, null, lower),
    result: [lower, upper],
  });
  return r.steps;
}

export const lowerUpperBound: TracerDef = {
  id: "lower-upper-bound",
  subject: "searching",
  compareGroup: "find",
  scene: "array",
  limits: SEARCH_INPUT,
  input: { kind: "sorted-target", pickTarget: pickSearchTarget },
  markers: MARKERS,
  narrationKeys: KEYS,
  counters: [{ slot: "comparisons", label: "probes" }],
  legend: ["pivot", "key", "sorted"],
  roleLabels: { pivot: "mid", key: "lower_bound", sorted: "target_copies" },
  vars: ["target", "lo", "hi", "mid", "lower"],
  trace,
};
