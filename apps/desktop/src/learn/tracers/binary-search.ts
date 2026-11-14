// Binary search. Mirrors content/learn/searching/binary-search/sample.{py,ts}: on sorted
// values, look at the middle of the range still possible and throw away the half that
// cannot hold the target.
import { pickSearchTarget, SEARCH_INPUT } from "../engine/search";
import { Recorder } from "../engine/recorder";
import type { NarrationKey, Pointer, Step, TracerDef } from "../engine/types";

type Key = NarrationKey<"binary-search">;
const KEYS = [
  "binary-search.ready",
  "binary-search.mid",
  "binary-search.found",
  "binary-search.go_right",
  "binary-search.go_left",
  "binary-search.missing",
] as const satisfies readonly Key[];
const say = (key: (typeof KEYS)[number], params?: Record<string, number>) => ({ key, params });

export const MARKERS = ["mid", "found", "compare", "missing"] as const;

export const ptrs = (lo: number, hi: number, mid: number | null, n: number): Pointer[] => [
  ...(lo < n ? [{ label: "lo", at: lo, tone: "i" } as Pointer] : []),
  ...(mid === null ? [] : [{ label: "mid", at: mid, tone: "pivot" } as Pointer]),
  ...(hi >= 0 ? [{ label: "hi", at: hi, tone: "j" } as Pointer] : []),
];

function trace(input: readonly number[]): Step[] {
  const target = input[0]!;
  const r = new Recorder(input.slice(1));
  const n = r.items.length;
  let lo = 0;
  let hi = n - 1;
  r.snap({ range: [lo, hi], pointers: ptrs(lo, hi, null, n), narration: say("binary-search.ready", { target, n }), marker: null, vars: { target, lo, hi, mid: null } });
  while (lo <= hi) {
    const mid = Math.floor((lo + hi) / 2);
    const range: [number, number] = [lo, hi];
    r.comparisons++;
    r.snap({
      roles: { [mid]: "pivot" },
      pointers: ptrs(lo, hi, mid, n),
      range,
      narration: say("binary-search.mid", { lo, hi, mid, value: r.value(mid), target }),
      marker: "mid",
      vars: { target, lo, hi, mid },
    });
    if (r.value(mid) === target) {
      r.snap({
        roles: { [mid]: "sorted" },
        pointers: ptrs(lo, hi, mid, n),
        range,
        narration: say("binary-search.found", { target, mid, checks: r.comparisons }),
        marker: "found",
        vars: { target, lo, hi, mid },
        result: [mid],
      });
      return r.steps;
    }
    const right = r.value(mid) < target;
    if (right) lo = mid + 1;
    else hi = mid - 1;
    r.snap({
      roles: { [mid]: "compare" },
      pointers: ptrs(lo, hi, mid, n),
      range: lo <= hi ? [lo, hi] : null,
      narration: say(right ? "binary-search.go_right" : "binary-search.go_left", { value: r.value(mid), target, lo, hi }),
      marker: "compare",
      vars: { target, lo, hi, mid },
    });
  }
  const all: Record<number, "out"> = {};
  for (let p = 0; p < n; p++) all[p] = "out";
  r.snap({ roles: all, pointers: ptrs(lo, hi, null, n), narration: say("binary-search.missing", { target, checks: r.comparisons }), marker: "missing", vars: { target, lo, hi, mid: null }, result: [-1] });
  return r.steps;
}

export const binarySearch: TracerDef = {
  id: "binary-search",
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
  vars: ["target", "lo", "hi", "mid"],
  trace,
};
