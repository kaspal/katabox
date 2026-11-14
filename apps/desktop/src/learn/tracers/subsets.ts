// All subsets by backtracking. Mirrors content/learn/backtracking/subsets/sample.{py,ts}:
// every call records the current choice as one subset, then tries adding each later
// value in turn (choose), explores, and takes it back (undo). Each subset appears exactly
// once, in this order: [], [a], [a, b], …. Output: how many, then each subset as its size
// followed by its values.
import { Recorder } from "../engine/recorder";
import type { NarrationKey, Step, TracerDef } from "../engine/types";
import { Outline } from "./outline";

type Key = NarrationKey<"subsets">;
const KEYS = ["subsets.ready", "subsets.record", "subsets.choose", "subsets.undo", "subsets.done"] as const satisfies readonly Key[];
const say = (key: (typeof KEYS)[number], params?: Record<string, number | string>) => ({ key, params: params as Record<string, number> });

export const MARKERS = ["record", "choose", "undo"] as const;
const show = (xs: readonly number[]) => `{${xs.join(", ")}}`;

function trace(values: readonly number[]): Step[] {
  const r = new Recorder([]);
  const outline = new Outline();
  const chosen: number[] = [];
  const out: number[][] = [];
  const snap = (narration: ReturnType<typeof say>, marker: string | null) =>
    r.snap({
      narration,
      marker,
      decision: outline.data(),
      slots: { layout: "row", label: "chosen", cells: chosen.map((value, id) => ({ id, value })), roles: chosen.length ? { [chosen.length - 1]: "key" } : {} },
      aux: { label: "found", cells: out.map((s, id) => ({ id, value: s.length, text: show(s) })), roles: {} },
      vars: { size: chosen.length, found: out.length },
    });
  const grow = (start: number, depth: number) => {
    const line = outline.open(depth, show(chosen));
    out.push([...chosen]);
    outline.mark(line, "sorted");
    r.writes++;
    snap(say("subsets.record", { set: show(chosen), count: out.length }), "record");
    for (let k = start; k < values.length; k++) {
      chosen.push(values[k]!);
      r.moves++;
      snap(say("subsets.choose", { value: values[k]!, k }), "choose");
      grow(k + 1, depth + 1);
      chosen.pop();
      outline.back(line);
      snap(say("subsets.undo", { value: values[k]! }), "undo");
    }
  };

  snap(say("subsets.ready", { n: values.length, total: 2 ** values.length }), null);
  grow(0, 0);
  snap(say("subsets.done", { count: out.length, n: values.length }), null);
  r.steps[r.steps.length - 1]!.result = [out.length, ...out.flatMap((s) => [s.length, ...s])];
  return r.steps;
}

export const subsets: TracerDef = {
  id: "subsets",
  subject: "backtracking",
  scene: "decision",
  limits: { min_len: 1, max_len: 4, min: 1, max: 9 },
  input: { kind: "values" },
  markers: MARKERS,
  narrationKeys: KEYS,
  counters: [
    { slot: "writes", label: "recorded" },
    { slot: "moves", label: "choices" },
  ],
  legend: ["key", "sorted"],
  roleLabels: { key: "current", sorted: "recorded" },
  vars: ["size", "found"],
  trace,
};
