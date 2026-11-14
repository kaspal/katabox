// Combination sum: every way to reach a target with candidates that may repeat. Mirrors
// content/learn/backtracking/combination-sum/sample.{py,ts}: sort the candidates; each
// call may add any candidate from `start` on (so combinations come out in non-decreasing
// order, never twice); as soon as a candidate exceeds what is left, the larger ones will
// too, so the loop stops (prune). Output: how many, then each combination as its size
// followed by its values.
import { int } from "../engine/inputs";
import { Recorder } from "../engine/recorder";
import type { NarrationKey, Step, TracerDef } from "../engine/types";
import { Outline } from "./outline";

type Key = NarrationKey<"combination-sum">;
const KEYS = ["combination-sum.ready", "combination-sum.choose", "combination-sum.found", "combination-sum.prune", "combination-sum.done"] as const satisfies readonly Key[];
const say = (key: (typeof KEYS)[number], params?: Record<string, number | string>) => ({ key, params: params as Record<string, number> });

export const MARKERS = ["found", "prune", "choose"] as const;
const show = (xs: readonly number[]) => (xs.length ? xs.join(" + ") : "∅");

function trace(input: readonly number[]): Step[] {
  const [target = 1, ...raw] = input;
  const candidates = [...raw].sort((a, b) => a - b);
  const r = new Recorder([]);
  const outline = new Outline();
  const chosen: number[] = [];
  const out: number[][] = [];
  const snap = (narration: ReturnType<typeof say>, marker: string | null, at: number | null = null) =>
    r.snap({
      narration,
      marker,
      decision: outline.data(),
      slots: { layout: "row", label: "candidates", cells: candidates.map((value, id) => ({ id, value })), roles: at === null ? {} : { [at]: "key" } },
      aux: { label: "found", cells: out.map((c, id) => ({ id, value: c.length, text: c.join("+") })), roles: {} },
      vars: { left: target - chosen.reduce((s, v) => s + v, 0), found: out.length },
    });
  const search = (start: number, left: number, depth: number) => {
    const line = outline.open(depth, `${show(chosen)} (${left})`);
    if (left === 0) {
      out.push([...chosen]);
      outline.mark(line, "sorted");
      r.writes++;
      snap(say("combination-sum.found", { combo: show(chosen), count: out.length }), "found");
      return;
    }
    for (let k = start; k < candidates.length; k++) {
      r.comparisons++;
      if (candidates[k]! > left) {
        outline.mark(line, "out");
        snap(say("combination-sum.prune", { value: candidates[k]!, left }), "prune", k);
        break;
      }
      chosen.push(candidates[k]!);
      r.moves++;
      outline.back(line);
      snap(say("combination-sum.choose", { value: candidates[k]!, left: left - candidates[k]! }), "choose", k);
      search(k, left - candidates[k]!, depth + 1);
      chosen.pop();
      outline.back(line);
    }
  };

  snap(say("combination-sum.ready", { target, n: candidates.length }), null);
  search(0, target, 0);
  snap(say("combination-sum.done", { count: out.length, target }), null);
  r.steps[r.steps.length - 1]!.result = [out.length, ...out.flatMap((c) => [c.length, ...c])];
  return r.steps;
}

export const combinationSum: TracerDef = {
  id: "combination-sum",
  subject: "backtracking",
  scene: "decision",
  limits: { min_len: 2, max_len: 5, min: 2, max: 12 },
  input: {
    kind: "target",
    targetLabel: "target",
    pickTarget: (_values, next) => int(next, 5, 12),
    checkTarget: (target) => (target >= 2 && target <= 12 ? null : { min: 2, max: 12 }),
  },
  presets: {
    "few-coins": () => [8, 2, 3, 5],
    random: (next) => {
      const pool = [2, 3, 4, 5, 6, 7];
      const values = pool.filter(() => next() < 0.5).slice(0, 4);
      return [int(next, 5, 12), ...(values.length ? values : [3, 4])];
    },
    "no-way": () => [7, 2, 4],
  },
  markers: MARKERS,
  narrationKeys: KEYS,
  counters: [
    { slot: "comparisons", label: "comparisons" },
    { slot: "moves", label: "choices" },
    { slot: "writes", label: "recorded" },
  ],
  legend: ["key", "sorted", "out"],
  roleLabels: { key: "current", sorted: "recorded", out: "pruned" },
  vars: ["left", "found"],
  trace,
};
