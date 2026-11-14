// Floyd's cycle detection (tortoise and hare). Mirrors
// content/learn/linear/floyd-cycle/sample.{py,ts}: slow moves one node, fast two; if fast
// runs off the end there is no cycle, if they meet there is one. Then slow restarts at the
// head and both move one node at a time: they meet again exactly where the cycle starts.
// The input is the cycle position (where the last node links back to, −1 for none) and the
// values; the tail's back link is drawn as an arc.
import { listData, type LNode } from "../engine/list";
import { Recorder } from "../engine/recorder";
import type { NarrationKey, Role, Step, TracerDef } from "../engine/types";

type Key = NarrationKey<"floyd-cycle">;
const KEYS = [
  "floyd-cycle.ready",
  "floyd-cycle.move",
  "floyd-cycle.meet",
  "floyd-cycle.no_cycle",
  "floyd-cycle.reset",
  "floyd-cycle.walk",
  "floyd-cycle.found",
] as const satisfies readonly Key[];
const say = (key: (typeof KEYS)[number], params?: Record<string, number | string>) => ({ key, params });

export const MARKERS = ["move", "meet", "no-cycle", "reset", "walk", "found"] as const;

function trace(input: readonly number[]): Step[] {
  const pos = input[0]!;
  const values = input.slice(1);
  const n = values.length;
  const r = new Recorder([]);
  const nodes = new Map<number, LNode>();
  values.forEach((value, id) => nodes.set(id, { id, value, next: id + 1 < n ? id + 1 : pos >= 0 ? pos : null, prev: null }));
  const order = values.map((_, id) => id);
  let slow: number | null = 0;
  let fast: number | null = 0;
  const next = (id: number | null) => (id === null ? null : nodes.get(id)!.next);
  const snap = (narration: ReturnType<typeof say>, marker: string | null, roles: Record<number, Role> = {}) =>
    r.snap({
      narration,
      marker,
      list: listData(nodes, order, {
        roles,
        labels: [
          ["slow", slow, "i"],
          ["fast", fast, "j"],
        ],
      }),
      vars: { pos, slow, fast },
    });

  snap(say("floyd-cycle.ready", { n, pos }), null);
  let met = false;
  while (fast !== null && next(fast) !== null) {
    slow = next(slow);
    fast = next(next(fast));
    r.comparisons++;
    snap(say("floyd-cycle.move", { slow: slow ?? "∅", fast: fast ?? "∅" }), "move", { ...(slow !== null ? { [slow]: "compare" } : {}), ...(fast !== null ? { [fast]: "pivot" } : {}) });
    if (slow === fast) {
      met = true;
      snap(say("floyd-cycle.meet", { at: slow! }), "meet", { [slow!]: "swap" });
      break;
    }
  }
  if (!met) {
    snap(say("floyd-cycle.no_cycle", { steps: r.comparisons }), "no-cycle");
    r.steps[r.steps.length - 1]!.result = [-1];
    return r.steps;
  }
  slow = 0;
  snap(say("floyd-cycle.reset"), "reset", { 0: "compare", [fast!]: "pivot" });
  while (slow !== fast) {
    slow = next(slow);
    fast = next(fast);
    r.moves++;
    snap(say("floyd-cycle.walk", { slow: slow!, fast: fast! }), "walk", { [slow!]: "compare", [fast!]: "pivot" });
  }
  snap(say("floyd-cycle.found", { at: slow!, value: values[slow!]! }), "found", { [slow!]: "sorted" });
  r.steps[r.steps.length - 1]!.result = [slow!];
  return r.steps;
}

export const floydCycle: TracerDef = {
  id: "floyd-cycle",
  subject: "linear",
  scene: "list",
  limits: { min_len: 2, max_len: 11, min: -1, max: 99 },
  input: {
    kind: "target",
    targetLabel: "cycle",
    pickTarget: (values, next) => (next() < 0.7 ? Math.floor(next() * values.length) : -1),
    checkTarget: (pos, values) => (pos >= -1 && pos < values.length ? null : { min: -1, max: values.length - 1 }),
  },
  markers: MARKERS,
  narrationKeys: KEYS,
  counters: [
    { slot: "comparisons", label: "hare_moves" },
    { slot: "moves", label: "second_walk" },
  ],
  legend: ["compare", "pivot", "swap", "sorted"],
  roleLabels: { compare: "slow", pivot: "fast", swap: "meeting", sorted: "cycle_start" },
  vars: ["pos", "slow", "fast"],
  trace,
};
