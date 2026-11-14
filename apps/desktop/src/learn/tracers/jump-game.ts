// Jump game. Mirrors content/learn/greedy/jump-game/sample.{py,ts}: from square i you may
// jump up to jumps[i] squares forward. Walk left to right keeping `reach`, the farthest
// square reachable so far; a square beyond reach means you are stuck. Output: [1 if the
// last square can be reached, else 0; the farthest square reached].
import { Recorder } from "../engine/recorder";
import type { NarrationKey, Pointer, Role, Step, TracerDef } from "../engine/types";

type Key = NarrationKey<"jump-game">;
const KEYS = ["jump-game.ready", "jump-game.step", "jump-game.farther", "jump-game.stuck", "jump-game.done"] as const satisfies readonly Key[];
const say = (key: (typeof KEYS)[number], params?: Record<string, number>) => ({ key, params });

export const MARKERS = ["stuck", "farther"] as const;

function trace(jumps: readonly number[]): Step[] {
  const n = jumps.length;
  const r = new Recorder(jumps);
  let reach = 0;
  const snap = (narration: ReturnType<typeof say>, marker: string | null, i: number | null) => {
    const roles: Record<number, Role> = {};
    for (let k = 0; k < n; k++) roles[k] = k <= reach ? "sorted" : "out";
    if (i !== null) roles[i] = i <= reach ? "key" : "swap";
    const pointers: Pointer[] = [{ label: "reach", at: Math.min(reach, n - 1), tone: "pivot" }];
    if (i !== null) pointers.push({ label: "i", at: i, tone: "key" });
    r.snap({ roles, pointers, narration, marker, vars: { i, reach } });
  };

  snap(say("jump-game.ready", { n }), null, null);
  for (let i = 0; i < n; i++) {
    r.comparisons++;
    if (i > reach) {
      snap(say("jump-game.stuck", { i, reach }), "stuck", i);
      r.steps[r.steps.length - 1]!.result = [0, reach];
      return r.steps;
    }
    if (i + jumps[i]! > reach) {
      reach = i + jumps[i]!;
      r.writes++;
      snap(say("jump-game.farther", { i, jump: jumps[i]!, reach }), "farther", i);
    } else {
      snap(say("jump-game.step", { i, jump: jumps[i]!, reach }), null, i);
    }
  }
  snap(say("jump-game.done", { last: n - 1, reach }), null, null);
  r.steps[r.steps.length - 1]!.result = [1, Math.min(reach, n - 1)];
  return r.steps;
}

export const jumpGame: TracerDef = {
  id: "jump-game",
  subject: "greedy",
  scene: "array",
  limits: { min_len: 1, max_len: 14, min: 0, max: 6 },
  input: { kind: "values" },
  presets: {
    "zero-trap": () => [2, 1, 1, 0, 3, 1],
    random: (next) => Array.from({ length: 5 + Math.floor(next() * 9) }, () => Math.floor(next() * 4)),
    "long-leaps": () => [2, 3, 1, 1, 0, 2, 4, 0, 1, 1],
  },
  markers: MARKERS,
  narrationKeys: KEYS,
  counters: [
    { slot: "comparisons", label: "values_read" },
    { slot: "writes", label: "records" },
  ],
  legend: ["key", "sorted", "out", "swap"],
  roleLabels: { key: "current", sorted: "reachable", out: "not_reached", swap: "stuck_here" },
  vars: ["i", "reach"],
  trace,
};
