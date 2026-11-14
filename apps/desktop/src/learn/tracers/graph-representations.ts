// Graph representations. Mirrors content/learn/graphs/graph-representations/sample.{py,ts}:
// one pass over the edge list fills an adjacency list (each node's neighbours, in edge
// order) and an adjacency matrix (the weight in row u, column v; 0 = no edge). An
// undirected edge is written both ways. Output: each node's neighbour count and list,
// then the matrix row by row.
import { decodeGraph, encodeGraph, graphCodec, graphData, randomGraph } from "../engine/graph";
import { Recorder } from "../engine/recorder";
import type { NarrationKey, Role, Step, TracerDef } from "../engine/types";

type Key = NarrationKey<"graph-representations">;
const KEYS = [
  "graph-representations.ready",
  "graph-representations.list",
  "graph-representations.matrix",
  "graph-representations.list_back",
  "graph-representations.done",
] as const satisfies readonly Key[];
const say = (key: (typeof KEYS)[number], params?: Record<string, number>) => ({ key, params });

export const MARKERS = ["list", "matrix", "list-back"] as const;
const codec = graphCodec({ directed: "choose", weighted: true, wMin: 1, wMax: 9 });

function trace(input: readonly number[]): Step[] {
  const g = decodeGraph(input);
  const r = new Recorder([]);
  const adj: number[][] = Array.from({ length: g.n }, () => []);
  const matrix: number[][] = Array.from({ length: g.n }, () => new Array<number>(g.n).fill(0));
  const done: Record<number, Role> = {};
  const snap = (narration: ReturnType<typeof say>, marker: string | null, edge?: number, row?: number, cell?: [number, number]) => {
    const edges: Record<number, Role> = { ...done };
    if (edge !== undefined) edges[edge] = "key";
    const nodes: Record<number, Role> = {};
    if (edge !== undefined) {
      nodes[g.edges[edge]!.u] = "key";
      nodes[g.edges[edge]!.v] = "compare";
    }
    r.snap({
      narration,
      marker,
      graph: {
        ...graphData(g, true, { nodes, edges }),
        adjacency: adj.map((l) => [...l]),
        hotRow: row,
        matrix: { label: "matrix", cells: matrix.map((m) => [...m]), hot: cell ? [cell] : [] },
      },
      vars: { u: edge === undefined ? null : g.edges[edge]!.u, v: edge === undefined ? null : g.edges[edge]!.v },
    });
  };

  snap(say("graph-representations.ready", { n: g.n, m: g.edges.length }), null);
  g.edges.forEach(({ u, v, w }, e) => {
    adj[u]!.push(v);
    r.writes++;
    snap(say("graph-representations.list", { u, v }), "list", e, u);
    matrix[u]![v] = w;
    r.writes++;
    snap(say("graph-representations.matrix", { u, v, w }), "matrix", e, undefined, [u, v]);
    if (!g.directed) {
      adj[v]!.push(u);
      matrix[v]![u] = w;
      r.writes += 2;
      snap(say("graph-representations.list_back", { u, v, w }), "list-back", e, v, [v, u]);
    }
    done[e] = "sorted";
  });
  const out = [...adj.flatMap((l) => [l.length, ...l]), ...matrix.flat()];
  snap(say("graph-representations.done", { n: g.n, list: adj.reduce((s, l) => s + l.length, 0), cells: g.n * g.n }), null);
  r.steps[r.steps.length - 1]!.result = out;
  return r.steps;
}

export const graphRepresentations: TracerDef = {
  id: "graph-representations",
  subject: "graphs",
  scene: "graph",
  limits: { min_len: 5, max_len: 62, min: 0, max: 12 },
  input: { kind: "codec", codec },
  presets: {
    random: (next) => randomGraph(next, { nMin: 4, nMax: 7, directed: next() < 0.5, weighted: true, density: 1.2, connected: true }),
    undirected: () => encodeGraph(5, false, [[0, 1, 4], [0, 2, 1], [1, 3, 2], [2, 3, 5], [3, 4, 3]]),
    directed: () => encodeGraph(5, true, [[0, 1, 4], [1, 2, 2], [2, 0, 7], [2, 3, 1], [3, 4, 6], [4, 3, 3]]),
  },
  markers: MARKERS,
  narrationKeys: KEYS,
  counters: [{ slot: "writes", label: "writes" }],
  legend: ["key", "compare", "sorted"],
  roleLabels: { key: "current", compare: "neighbour", sorted: "written" },
  vars: ["u", "v"],
  trace,
};
