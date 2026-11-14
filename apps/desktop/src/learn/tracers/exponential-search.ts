// Exponential search. Mirrors content/learn/searching/exponential-search/sample.{py,ts}:
// double a bound (1, 2, 4, …) until it passes the target or the end, then binary search
// between the last two bounds. Good when the target sits near the front or the length is
// unknown.
import { pickSearchTarget, SEARCH_INPUT } from "../engine/search";
import { Recorder } from "../engine/recorder";
import type { NarrationKey, Pointer, Step, TracerDef } from "../engine/types";
import { ptrs } from "./binary-search";

type Key = NarrationKey<"exponential-search">;
const KEYS = [
  "exponential-search.ready",
  "exponential-search.first_yes",
  "exponential-search.first_no",
  "exponential-search.double",
  "exponential-search.stop",
  "exponential-search.range",
  "exponential-search.mid",
  "exponential-search.found",
  "exponential-search.go_right",
  "exponential-search.go_left",
  "exponential-search.missing",
] as const satisfies readonly Key[];
const say = (key: (typeof KEYS)[number], params?: Record<string, number>) => ({ key, params });

export const MARKERS = ["first", "double", "range", "mid", "found", "compare", "missing"] as const;

function trace(input: readonly number[]): Step[] {
  const target = input[0]!;
  const r = new Recorder(input.slice(1));
  const n = r.items.length;
  const none = { target, bound: null, lo: null, hi: null, mid: null };
  r.snap({ narration: say("exponential-search.ready", { target, n }), marker: null, vars: none });
  r.comparisons++;
  if (r.value(0) === target) {
    r.snap({ roles: { 0: "sorted" }, narration: say("exponential-search.first_yes", { target }), marker: "first", vars: none, result: [0] });
    return r.steps;
  }
  r.snap({ roles: { 0: "compare" }, narration: say("exponential-search.first_no", { value: r.value(0), target }), marker: "first", vars: none });
  let bound = 1;
  while (bound < n) {
    r.comparisons++;
    const value = r.value(bound);
    const more = value < target;
    const b: Pointer = { label: "bound", at: bound, tone: "key" };
    r.snap({
      roles: { [bound]: more ? "compare" : "pivot" },
      pointers: [b],
      narration: say(more ? "exponential-search.double" : "exponential-search.stop", { bound, value, target, next: bound * 2 }),
      marker: "double",
      vars: { ...none, bound },
    });
    if (!more) break;
    bound *= 2;
  }
  let lo = Math.floor(bound / 2);
  let hi = Math.min(bound, n - 1);
  r.snap({ range: [lo, hi], pointers: ptrs(lo, hi, null, n), narration: say("exponential-search.range", { lo, hi, bound }), marker: "range", vars: { target, bound, lo, hi, mid: null } });
  while (lo <= hi) {
    const mid = Math.floor((lo + hi) / 2);
    const range: [number, number] = [lo, hi];
    r.comparisons++;
    r.snap({ roles: { [mid]: "pivot" }, pointers: ptrs(lo, hi, mid, n), range, narration: say("exponential-search.mid", { mid, value: r.value(mid), target }), marker: "mid", vars: { target, bound, lo, hi, mid } });
    if (r.value(mid) === target) {
      r.snap({ roles: { [mid]: "sorted" }, pointers: ptrs(lo, hi, mid, n), range, narration: say("exponential-search.found", { target, mid, checks: r.comparisons }), marker: "found", vars: { target, bound, lo, hi, mid }, result: [mid] });
      return r.steps;
    }
    const right = r.value(mid) < target;
    if (right) lo = mid + 1;
    else hi = mid - 1;
    r.snap({
      roles: { [mid]: "compare" },
      pointers: ptrs(lo, hi, mid, n),
      range: lo <= hi ? [lo, hi] : null,
      narration: say(right ? "exponential-search.go_right" : "exponential-search.go_left", { value: r.value(mid), target }),
      marker: "compare",
      vars: { target, bound, lo, hi, mid },
    });
  }
  r.snap({ narration: say("exponential-search.missing", { target, checks: r.comparisons }), marker: "missing", vars: { target, bound, lo, hi, mid: null }, result: [-1] });
  return r.steps;
}

export const exponentialSearch: TracerDef = {
  id: "exponential-search",
  subject: "searching",
  compareGroup: "find",
  scene: "array",
  limits: SEARCH_INPUT,
  input: { kind: "sorted-target", pickTarget: pickSearchTarget },
  markers: MARKERS,
  narrationKeys: KEYS,
  counters: [{ slot: "comparisons", label: "probes" }],
  legend: ["pivot", "compare", "sorted", "out"],
  roleLabels: { pivot: "mid", compare: "ruled_out_half", sorted: "found", out: "ruled_out" },
  vars: ["target", "bound", "lo", "hi", "mid"],
  trace,
};
