// Bellman-Ford shortest paths from node 0, negative weights allowed. Mirrors
// content/learn/graphs/bellman-ford/sample.{py,ts}: up to n − 1 rounds, each relaxing
// every edge in list order, stopping early after a round with no change; one more pass
// that still lowers a distance proves a negative cycle reachable from 0. Output: [1] for a
// negative cycle, else [0] followed by the distances (999 when unreachable).
import { decodeGraph, encodeGraph, graphCodec, graphData, randomGraph } from "../engine/graph";
import { Recorder } from "../engine/recorder";
import type { NarrationKey, NarrationParams, Role, Step, TracerDef } from "../engine/types";

type Key = NarrationKey<"bellman-ford">;
const KEYS = [
  "bellman-ford.ready",
  "bellman-ford.round",
  "bellman-ford.relax",
  "bellman-ford.settled",
  "bellman-ford.check",
  "bellman-ford.negative_cycle",
  "bellman-ford.done",
] as const satisfies readonly Key[];
const say = (key: (typeof KEYS)[number], params?: NarrationParams<number>) => ({ key, params });

export const MARKERS = ["relax", "settled", "negative-cycle"] as const;
export const INF = 999;
const codec = graphCodec({ directed: true, weighted: true, wMin: -5, wMax: 9 });

function trace(input: readonly number[]): Step[] {
  const g = decodeGraph(input);
  const r = new Recorder([]);
  const dist = new Array<number>(g.n).fill(INF);
  dist[0] = 0;
  let round = 0;
  const snap = (narration: ReturnType<typeof say>, marker: string | null, e?: number, role: Role = "swap") => {
    const nodes: Record<number, Role> = {};
    dist.forEach((d, v) => {
      if (d !== INF) nodes[v] = "sorted";
    });
    const edges: Record<number, Role> = {};
    if (e !== undefined) {
      edges[e] = role;
      nodes[g.edges[e]!.u] = "key";
      nodes[g.edges[e]!.v] = role;
    }
    r.snap({
      narration,
      marker,
      graph: graphData(g, true, { nodes, edges, notes: Object.fromEntries(dist.map((d, v) => [v, d === INF ? "∞" : String(d)])) }),
      vars: { round, edge: e === undefined ? null : e + 1 },
    });
  };

  snap(say("bellman-ford.ready", { n: g.n, m: g.edges.length, rounds: g.n - 1 }), null);
  for (round = 1; round <= g.n - 1; round++) {
    let changed = false;
    snap(say("bellman-ford.round", { round, of: g.n - 1 }), null);
    g.edges.forEach(({ u, v, w }, e) => {
      r.comparisons++;
      if (dist[u] !== INF && dist[u]! + w < dist[v]!) {
        dist[v] = dist[u]! + w;
        changed = true;
        r.writes++;
        snap(say("bellman-ford.relax", { u, v, w, d: dist[v]! }), "relax", e);
      }
    });
    if (!changed) {
      snap(say("bellman-ford.settled", { round }), "settled");
      break;
    }
  }
  round = Math.min(round, g.n - 1);
  snap(say("bellman-ford.check"), null);
  for (const [e, { u, v, w }] of g.edges.entries()) {
    if (dist[u] !== INF && dist[u]! + w < dist[v]!) {
      snap(say("bellman-ford.negative_cycle", { u, v, w }), "negative-cycle", e, "compare");
      r.steps[r.steps.length - 1]!.result = [1];
      return r.steps;
    }
  }
  snap(say("bellman-ford.done", { relaxed: r.writes, checked: r.comparisons }), null);
  r.steps[r.steps.length - 1]!.result = [0, ...dist];
  return r.steps;
}

export const bellmanFord: TracerDef = {
  id: "bellman-ford",
  subject: "graphs",
  scene: "graph",
  limits: { min_len: 5, max_len: 62, min: -5, max: 12 },
  input: { kind: "codec", codec },
  presets: {
    "negative-edge": () => encodeGraph(5, true, [[3, 4, 2], [1, 3, -2], [0, 1, 4], [0, 2, 1], [2, 1, 2], [2, 3, 5]]),
    random: (next) => randomGraph(next, { nMin: 4, nMax: 7, directed: true, weighted: true, wMin: -2, wMax: 9, density: 1.4, connected: true, dag: true }),
    "negative-cycle": () => encodeGraph(5, true, [[0, 1, 3], [1, 2, 2], [2, 3, -4], [3, 1, 1], [3, 4, 2]]),
  },
  markers: MARKERS,
  narrationKeys: KEYS,
  counters: [
    { slot: "comparisons", label: "edges_checked" },
    { slot: "writes", label: "relaxed" },
  ],
  legend: ["key", "swap", "compare", "sorted"],
  roleLabels: { key: "current", swap: "relaxed", compare: "still_lowers", sorted: "reached" },
  vars: ["round", "edge"],
  trace,
};
