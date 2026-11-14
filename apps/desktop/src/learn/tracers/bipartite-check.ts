// Bipartite check by two-colouring. Mirrors content/learn/graphs/bipartite-check/sample.{py,ts}:
// BFS from each uncoloured node, giving every new neighbour the other side; an edge whose
// two ends got the same side closes an odd cycle, so no split exists. Output: each node's
// side (0 or 1), or [-1]. Sides are also written under the nodes (A / B).
import { adjacency, decodeGraph, encodeGraph, graphCodec, graphData, randomGraph } from "../engine/graph";
import { Recorder } from "../engine/recorder";
import type { NarrationKey, NarrationParams, Role, Step, TracerDef } from "../engine/types";

type Key = NarrationKey<"bipartite-check">;
const KEYS = [
  "bipartite-check.ready",
  "bipartite-check.start",
  "bipartite-check.colour",
  "bipartite-check.fine",
  "bipartite-check.clash",
  "bipartite-check.done",
] as const satisfies readonly Key[];
const say = (key: (typeof KEYS)[number], params?: NarrationParams<number>) => ({ key, params });

export const MARKERS = ["start", "colour", "clash"] as const;
const codec = graphCodec({ directed: false, weighted: false });

function trace(input: readonly number[]): Step[] {
  const g = decodeGraph(input);
  const adj = adjacency(g);
  const r = new Recorder([]);
  const side = new Array<number>(g.n).fill(-1);
  const queue: number[] = [];
  let head = 0;
  const snap = (narration: ReturnType<typeof say>, marker: string | null, u: number | null, look?: { v: number; e: number; role: Role }) => {
    const nodes: Record<number, Role> = {};
    side.forEach((s, v) => {
      if (s === 0) nodes[v] = "pivot";
      if (s === 1) nodes[v] = "sorted";
    });
    if (u !== null) nodes[u] = "key";
    const edges: Record<number, Role> = {};
    if (look) {
      edges[look.e] = look.role;
      if (look.role === "swap") nodes[look.v] = "swap";
    }
    r.snap({
      narration,
      marker,
      graph: graphData(g, false, { nodes, edges, notes: Object.fromEntries(side.map((s, v) => [v, s === 0 ? "A" : s === 1 ? "B" : ""]).filter(([, s]) => s !== "")) }),
      slots: { layout: "row", label: "queue", cells: queue.slice(head).map((v) => ({ id: v, value: v })) },
      vars: { u },
    });
  };

  snap(say("bipartite-check.ready", { n: g.n }), null, null);
  for (let s = 0; s < g.n; s++) {
    if (side[s] !== -1) continue;
    side[s] = 0;
    queue.push(s);
    snap(say("bipartite-check.start", { s }), "start", s);
    while (head < queue.length) {
      const u = queue[head++]!;
      for (const { v, e } of adj[u]!) {
        r.comparisons++;
        if (side[v] === -1) {
          side[v] = 1 - side[u]!;
          queue.push(v);
          r.writes++;
          snap(say("bipartite-check.colour", { u, v, side: side[v]! }), "colour", u, { v, e, role: "sorted" });
        } else if (side[v] === side[u]) {
          snap(say("bipartite-check.clash", { u, v }), "clash", u, { v, e, role: "swap" });
          r.steps[r.steps.length - 1]!.result = [-1];
          return r.steps;
        } else {
          snap(say("bipartite-check.fine", { u, v }), null, u, { v, e, role: "compare" });
        }
      }
    }
  }
  snap(say("bipartite-check.done", { a: side.filter((s) => s === 0).length, b: side.filter((s) => s === 1).length }), null, null);
  r.steps[r.steps.length - 1]!.result = side;
  return r.steps;
}

export const bipartiteCheck: TracerDef = {
  id: "bipartite-check",
  subject: "graphs",
  scene: "graph",
  limits: { min_len: 5, max_len: 62, min: 0, max: 12 },
  input: { kind: "codec", codec },
  presets: {
    "even-cycles": () => encodeGraph(8, false, [[0, 1], [1, 2], [2, 3], [3, 0], [2, 4], [4, 5], [5, 6], [6, 7], [7, 4]]),
    random: (next) => randomGraph(next, { nMin: 5, nMax: 9, directed: false, weighted: false, density: 1.1, connected: true }),
    "odd-cycle": () => encodeGraph(6, false, [[0, 1], [1, 2], [2, 3], [3, 4], [4, 0], [2, 5]]),
  },
  markers: MARKERS,
  narrationKeys: KEYS,
  counters: [
    { slot: "comparisons", label: "edges_followed" },
    { slot: "writes", label: "labelled" },
  ],
  legend: ["key", "pivot", "sorted", "compare", "swap"],
  roleLabels: { key: "current", pivot: "side_a", sorted: "side_b", compare: "fine_edge", swap: "clash" },
  vars: ["u"],
  trace,
};
