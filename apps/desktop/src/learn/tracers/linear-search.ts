// Linear search. Mirrors content/learn/searching/linear-search/sample.{py,ts}: look at every
// position in turn until the target turns up; the order of the values does not matter.
import { pickSearchTarget, SEARCH_INPUT } from "../engine/search";
import { Recorder } from "../engine/recorder";
import type { NarrationKey, NarrationParams, Step, TracerDef } from "../engine/types";

type Key = NarrationKey<"linear-search">;
const KEYS = [
  "linear-search.ready",
  "linear-search.check_no",
  "linear-search.check_yes",
  "linear-search.found",
  "linear-search.missing",
] as const satisfies readonly Key[];
const say = (key: (typeof KEYS)[number], params?: NarrationParams<number>) => ({ key, params });

export const MARKERS = ["check", "found", "missing"] as const;

function trace(input: readonly number[]): Step[] {
  const target = input[0]!;
  const r = new Recorder(input.slice(1));
  const n = r.items.length;
  r.snap({ narration: say("linear-search.ready", { target, n }), marker: null, vars: { target, i: null } });
  let answer = -1;
  for (let i = 0; i < n; i++) {
    r.comparisons++;
    const hit = r.value(i) === target;
    const seen: Record<number, "out"> = {};
    for (let p = 0; p < i; p++) seen[p] = "out";
    r.snap({
      roles: { ...seen, [i]: "compare" },
      pointers: [{ label: "i", at: i, tone: "i" }],
      narration: say(hit ? "linear-search.check_yes" : "linear-search.check_no", { value: r.value(i), target, i }),
      marker: "check",
      vars: { target, i },
    });
    if (hit) {
      answer = i;
      r.snap({
        roles: { ...seen, [i]: "sorted" },
        pointers: [{ label: "i", at: i, tone: "i" }],
        narration: say("linear-search.found", { target, i, checks: r.comparisons }),
        marker: "found",
        vars: { target, i },
        result: [i],
      });
      break;
    }
  }
  if (answer === -1) {
    const all: Record<number, "out"> = {};
    for (let p = 0; p < n; p++) all[p] = "out";
    r.snap({ roles: all, narration: say("linear-search.missing", { target, checks: r.comparisons }), marker: "missing", vars: { target, i: null }, result: [-1] });
  }
  return r.steps;
}

export const linearSearch: TracerDef = {
  id: "linear-search",
  subject: "searching",
  compareGroup: "find",
  scene: "array",
  limits: SEARCH_INPUT,
  input: { kind: "target", pickTarget: pickSearchTarget },
  markers: MARKERS,
  narrationKeys: KEYS,
  counters: [{ slot: "comparisons", label: "probes" }],
  legend: ["compare", "sorted", "out"],
  roleLabels: { sorted: "found", out: "ruled_out" },
  vars: ["target", "i"],
  trace,
};
