// Climbing stairs: how many ways to climb n steps taking 1 or 2 at a time. Mirrors
// content/learn/dynamic-programming/climbing-stairs/sample.{py,ts}: the last move onto
// step i came from step i − 1 or from step i − 2, so ways[i] = ways[i − 1] + ways[i − 2],
// with ways[0] = 1 (standing still is one way). Output: the whole table, ways[0..n].
import { Recorder } from "../engine/recorder";
import type { NarrationKey, Step, TracerDef } from "../engine/types";

type Key = NarrationKey<"climbing-stairs">;
const KEYS = ["climbing-stairs.ready", "climbing-stairs.one", "climbing-stairs.two", "climbing-stairs.done"] as const satisfies readonly Key[];
const say = (key: (typeof KEYS)[number], params?: Record<string, number>) => ({ key, params });

export const MARKERS = ["one", "two"] as const;

function trace(input: readonly number[]): Step[] {
  const n = input[0]!;
  const r = new Recorder([]);
  const ways: (number | null)[] = new Array(n + 1).fill(null);
  ways[0] = 1;
  const snap = (narration: ReturnType<typeof say>, marker: string | null, hot: number | null = null, deps: number[] = []) =>
    r.snap({
      narration,
      marker,
      dp: { label: "ways", colLabels: Array.from({ length: n + 1 }, (_, k) => String(k)), cells: [[...ways]], hot: hot === null ? null : [0, hot], deps: deps.map((d) => [0, d]) },
      vars: { i: hot, ways: hot === null ? null : ways[hot]! },
    });

  snap(say("climbing-stairs.ready", { n }), null);
  for (let i = 1; i <= n; i++) {
    ways[i] = ways[i - 1]!;
    r.writes++;
    snap(say("climbing-stairs.one", { i, from: i - 1, ways: ways[i]! }), "one", i, [i - 1]);
    if (i >= 2) {
      ways[i]! += ways[i - 2]!;
      r.writes++;
      snap(say("climbing-stairs.two", { i, from: i - 2, add: ways[i - 2]!, ways: ways[i]! }), "two", i, [i - 2]);
    }
  }
  snap(say("climbing-stairs.done", { n, ways: ways[n]! }), null);
  r.steps[r.steps.length - 1]!.result = ways.map((v) => v!);
  return r.steps;
}

export const climbingStairs: TracerDef = {
  id: "climbing-stairs",
  subject: "dynamic-programming",
  scene: "dp",
  limits: { min_len: 1, max_len: 1, min: 1, max: 15 },
  input: { kind: "count" },
  markers: MARKERS,
  narrationKeys: KEYS,
  counters: [{ slot: "writes", label: "writes" }],
  legend: ["key", "compare", "sorted"],
  roleLabels: { key: "current", compare: "read", sorted: "filled" },
  vars: ["i", "ways"],
  trace,
};
