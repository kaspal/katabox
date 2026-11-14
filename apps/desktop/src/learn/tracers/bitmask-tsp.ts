// Travelling salesperson by bitmask DP (Held-Karp). Mirrors
// content/learn/dynamic-programming/bitmask-tsp/sample.{py,ts}: best[mask][last] is the
// shortest route that starts at city 0, visits exactly the cities in `mask` (a set
// written as bits) and stops at `last`. Masks are taken in increasing order, so every
// smaller set is finished first; each route is extended by one unvisited city. The
// answer closes the loop back to 0. Output: [the shortest round trip].
import { matrixCodec } from "../engine/shapes";
import { Recorder } from "../engine/recorder";
import type { NarrationKey, NarrationParams, Step, TracerDef } from "../engine/types";

type Key = NarrationKey<"bitmask-tsp">;
const KEYS = ["bitmask-tsp.ready", "bitmask-tsp.extend", "bitmask-tsp.close", "bitmask-tsp.done"] as const satisfies readonly Key[];
const say = (key: (typeof KEYS)[number], params?: NarrationParams) => ({ key, params: params as NarrationParams<number> });

export const MARKERS = ["extend"] as const;
export const INF = 1e6;
export const MAX_CITIES = 5;

const bits = (mask: number, n: number) =>
  Array.from({ length: n }, (_, k) => ((mask >> (n - 1 - k)) & 1 ? "1" : "0")).join("");

function trace(input: readonly number[]): Step[] {
  const n = input[0]!;
  const d = (i: number, j: number) => input[1 + i * n + j]!;
  const full = 1 << n;
  const r = new Recorder([]);
  const best: number[][] = Array.from({ length: full }, () => new Array<number>(n).fill(INF));
  best[1]![0] = 0;
  // Only masks with city 0 are ever reached: they are the rows shown.
  const masks = Array.from({ length: full }, (_, m) => m).filter((m) => m & 1);
  const snap = (narration: ReturnType<typeof say>, marker: string | null, hot: [number, number] | null = null, deps: [number, number][] = []) =>
    r.snap({
      narration,
      marker,
      dp: {
        label: "route_length",
        colLabels: Array.from({ length: n }, (_, k) => String(k)),
        rowLabels: masks.map((m) => bits(m, n)),
        cells: masks.map((m) => best[m]!.map((v) => (v >= INF ? null : v))),
        hot: hot ? [masks.indexOf(hot[0]), hot[1]] : null,
        deps: deps.map(([m, c]) => [masks.indexOf(m), c]),
      },
      vars: { mask: hot ? bits(hot[0], n) : null, last: hot ? hot[1] : null },
    });

  snap(say("bitmask-tsp.ready", { n, rows: masks.length }), null);
  for (let mask = 1; mask < full; mask += 2) {
    for (let last = 0; last < n; last++) {
      if (best[mask]![last] === INF) continue;
      for (let city = 0; city < n; city++) {
        if (mask & (1 << city)) continue;
        const next = mask | (1 << city);
        const total = best[mask]![last]! + d(last, city);
        r.comparisons++;
        if (total < best[next]![city]!) {
          best[next]![city] = total;
          r.writes++;
          snap(say("bitmask-tsp.extend", { last, city, total, set: bits(next, n) }), "extend", [next, city], [[mask, last]]);
        }
      }
    }
  }
  let answer = INF;
  let closeFrom = -1;
  for (let last = 1; last < n; last++) {
    const total = best[full - 1]![last]! + d(last, 0);
    if (total < answer) {
      answer = total;
      closeFrom = last;
    }
  }
  snap(say("bitmask-tsp.close", { last: closeFrom, back: d(closeFrom, 0), total: answer }), null, [full - 1, closeFrom]);
  snap(say("bitmask-tsp.done", { n, total: answer }), null);
  r.steps[r.steps.length - 1]!.result = [answer];
  return r.steps;
}

export const enc = (rows: number[][]) => [rows.length, ...rows.flat()];

export const bitmaskTsp: TracerDef = {
  id: "bitmask-tsp",
  subject: "dynamic-programming",
  scene: "dp",
  limits: { min_len: 5, max_len: 26, min: 0, max: 20 },
  input: { kind: "codec", codec: matrixCodec({ min: 2, max: MAX_CITIES, weight: 20 }) },
  presets: {
    "four-towns": () => enc([[0, 2, 9, 10], [2, 0, 6, 4], [9, 6, 0, 3], [10, 4, 3, 0]]),
    random: (next) => {
      const n = 3 + Math.floor(next() * (MAX_CITIES - 2));
      const rows = Array.from({ length: n }, () => new Array<number>(n).fill(0));
      for (let i = 0; i < n; i++)
        for (let j = i + 1; j < n; j++) {
          const w = 1 + Math.floor(next() * 20);
          rows[i]![j] = w;
          rows[j]![i] = w;
        }
      return enc(rows);
    },
    "five-towns": () => enc([[0, 3, 4, 2, 7], [3, 0, 4, 6, 3], [4, 4, 0, 5, 8], [2, 6, 5, 0, 6], [7, 3, 8, 6, 0]]),
  },
  markers: MARKERS,
  narrationKeys: KEYS,
  counters: [
    { slot: "comparisons", label: "comparisons" },
    { slot: "writes", label: "writes" },
  ],
  legend: ["key", "compare", "sorted"],
  roleLabels: { key: "current", compare: "read", sorted: "filled" },
  vars: ["mask", "last"],
  trace,
};
