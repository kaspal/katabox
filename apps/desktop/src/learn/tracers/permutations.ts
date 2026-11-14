// All orderings by backtracking. Mirrors content/learn/backtracking/permutations/sample.{py,ts}:
// fill the positions one by one with any value not used yet (by position in the input,
// so equal values give repeated orderings); a full row is one permutation. Output: how
// many, then each permutation's values in turn.
import { Recorder } from "../engine/recorder";
import type { NarrationKey, NarrationParams, Step, TracerDef } from "../engine/types";
import { Outline } from "./outline";

type Key = NarrationKey<"permutations">;
const KEYS = ["permutations.ready", "permutations.choose", "permutations.record", "permutations.undo", "permutations.done"] as const satisfies readonly Key[];
const say = (key: (typeof KEYS)[number], params?: NarrationParams) => ({ key, params: params as NarrationParams<number> });

export const MARKERS = ["record", "choose", "undo"] as const;
const show = (xs: readonly number[]) => `[${xs.join(", ")}]`;

function trace(values: readonly number[]): Step[] {
  const n = values.length;
  const r = new Recorder([]);
  const outline = new Outline();
  const used = new Array<boolean>(n).fill(false);
  const chosen: number[] = [];
  const out: number[][] = [];
  const snap = (narration: ReturnType<typeof say>, marker: string | null) =>
    r.snap({
      narration,
      marker,
      decision: outline.data(),
      slots: { layout: "row", label: "values", cells: values.map((value, id) => ({ id, value })), roles: Object.fromEntries(used.map((u, k) => [k, u ? "out" : "idle"])) },
      aux: { label: "found", cells: out.map((p, id) => ({ id, value: p[0]!, text: p.join("") })), roles: {} },
      vars: { placed: chosen.length, found: out.length },
    });
  const place = (depth: number) => {
    const line = outline.open(depth, show(chosen));
    if (chosen.length === n) {
      out.push([...chosen]);
      outline.mark(line, "sorted");
      r.writes++;
      snap(say("permutations.record", { perm: show(chosen), num: out.length }), "record");
      return;
    }
    for (let k = 0; k < n; k++) {
      if (!used[k]) {
        used[k] = true;
        chosen.push(values[k]!);
        r.moves++;
        snap(say("permutations.choose", { value: values[k]!, at: chosen.length - 1 }), "choose");
        place(depth + 1);
        chosen.pop();
        used[k] = false;
        outline.back(line);
        snap(say("permutations.undo", { value: values[k]! }), "undo");
      }
    }
  };

  snap(say("permutations.ready", { n }), null);
  place(0);
  snap(say("permutations.done", { num: out.length, n }), null);
  r.steps[r.steps.length - 1]!.result = [out.length, ...out.flat()];
  return r.steps;
}

export const permutations: TracerDef = {
  id: "permutations",
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
  legend: ["key", "sorted", "out"],
  roleLabels: { key: "current", sorted: "recorded", out: "used" },
  vars: ["placed", "found"],
  trace,
};
