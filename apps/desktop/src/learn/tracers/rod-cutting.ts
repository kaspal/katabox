// Rod cutting (unbounded knapsack). Mirrors content/learn/dynamic-programming/rod-cutting/sample.{py,ts}:
// prices[k] is what a piece of length k + 1 sells for; best[L] is the most a rod of
// length L can earn: try every first piece `cut` and add the best for what is left. The
// same length may be cut many times (unlimited "items"). Following the first cuts back
// gives the pieces. Output: the best total, then the piece lengths.
import { int } from "../engine/inputs";
import { Recorder } from "../engine/recorder";
import type { NarrationKey, Role, Step, TracerDef } from "../engine/types";

type Key = NarrationKey<"rod-cutting">;
const KEYS = ["rod-cutting.ready", "rod-cutting.length", "rod-cutting.improve", "rod-cutting.trace", "rod-cutting.done"] as const satisfies readonly Key[];
const say = (key: (typeof KEYS)[number], params?: Record<string, number>) => ({ key, params });

export const MARKERS = ["improve", "trace"] as const;

function trace(prices: readonly number[]): Step[] {
  const n = prices.length;
  const r = new Recorder([]);
  const best: (number | null)[] = [0, ...new Array<number | null>(n).fill(null)];
  const first = new Array<number>(n + 1).fill(0);
  const pieces: number[] = [];
  const path: [number, number][] = [];
  const snap = (narration: ReturnType<typeof say>, marker: string | null, hot: number | null = null, deps: number[] = [], piece: number | null = null) => {
    const roles: Record<number, Role> = {};
    for (const p of pieces) roles[p - 1] = "swap";
    if (piece !== null) roles[piece - 1] = "key";
    r.snap({
      narration,
      marker,
      slots: { layout: "row", label: "prices", cells: prices.map((value, id) => ({ id, value, text: `${id + 1}:${value}` })), roles },
      dp: { label: "best_value", colLabels: Array.from({ length: n + 1 }, (_, k) => String(k)), cells: [[...best]], hot: hot === null ? null : [0, hot], deps: deps.map((d) => [0, d]), path: [...path] },
      vars: { length: hot, best: hot === null ? null : best[hot] },
    });
  };

  snap(say("rod-cutting.ready", { n }), null);
  for (let length = 1; length <= n; length++) {
    best[length] = 0;
    snap(say("rod-cutting.length", { length }), null, length);
    for (let cut = 1; cut <= length; cut++) {
      r.comparisons++;
      const value = prices[cut - 1]! + best[length - cut]!;
      if (value > best[length]!) {
        best[length] = value;
        first[length] = cut;
        r.writes++;
        snap(say("rod-cutting.improve", { length, cut, rest: length - cut, value }), "improve", length, [length - cut], cut);
      }
    }
  }
  let length = n;
  while (length > 0) {
    pieces.push(first[length]!);
    path.push([0, length]);
    snap(say("rod-cutting.trace", { piece: first[length]!, left: length - first[length]! }), "trace", null, [], first[length]!);
    length -= first[length]!;
  }
  snap(say("rod-cutting.done", { n, best: best[n]!, count: pieces.length }), null);
  r.steps[r.steps.length - 1]!.result = [best[n]!, ...pieces];
  return r.steps;
}

export const rodCutting: TracerDef = {
  id: "rod-cutting",
  subject: "dynamic-programming",
  scene: "dp",
  limits: { min_len: 1, max_len: 10, min: 1, max: 30 },
  input: { kind: "values" },
  presets: {
    "cut-pays": () => [2, 5, 7, 8, 12, 13],
    random: (next) => {
      const prices: number[] = [];
      const n = int(next, 3, 10);
      for (let k = 0; k < n; k++) prices.push(Math.min(30, (prices[k - 1] ?? 0) + int(next, 1, 5)));
      return prices;
    },
    "whole-rod": () => [1, 2, 3, 4, 5, 6, 7, 20],
  },
  markers: MARKERS,
  narrationKeys: KEYS,
  counters: [
    { slot: "comparisons", label: "comparisons" },
    { slot: "writes", label: "writes" },
  ],
  legend: ["key", "compare", "sorted", "swap"],
  roleLabels: { key: "current", compare: "read", sorted: "filled", swap: "taken" },
  vars: ["length", "best"],
  trace,
};
