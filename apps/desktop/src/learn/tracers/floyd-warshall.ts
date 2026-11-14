// Floyd-Warshall all-pairs shortest paths. Mirrors content/learn/graphs/floyd-warshall/sample.{py,ts}:
// start from the direct edges; then, for each node k in turn, allow paths that pass
// through k: dist[i][j] improves when dist[i][k] + dist[k][j] is smaller. Output: the
// final matrix row by row (999 = no path).
import { decodeGraph, encodeGraph, graphCodec, graphData, randomGraph } from "../engine/graph";
import { Recorder } from "../engine/recorder";
import type { NarrationKey, Role, Step, TracerDef } from "../engine/types";

type Key = NarrationKey<"floyd-warshall">;
const KEYS = [
  "floyd-warshall.ready",
  "floyd-warshall.pivot",
  "floyd-warshall.improve",
  "floyd-warshall.improve_new",
  "floyd-warshall.done",
] as const satisfies readonly Key[];
const say = (key: (typeof KEYS)[number], params?: Record<string, number>) => ({ key, params });

export const MARKERS = ["pivot", "improve"] as const;
export const INF = 999;
export const MAX_FW_NODES = 6;
const codec = graphCodec({ directed: true, weighted: true, wMin: 1, wMax: 9, maxNodes: MAX_FW_NODES, maxEdges: 15 });

function trace(input: readonly number[]): Step[] {
  const g = decodeGraph(input);
  const n = g.n;
  const r = new Recorder([]);
  const dist = Array.from({ length: n }, (_, i) => Array.from({ length: n }, (_, j): number => (i === j ? 0 : INF)));
  for (const { u, v, w } of g.edges) dist[u]![v] = w;
  let k: number | null = null;
  const snap = (narration: ReturnType<typeof say>, marker: string | null, cell?: [number, number]) => {
    const nodes: Record<number, Role> = {};
    if (k !== null) nodes[k] = "pivot";
    if (cell) {
      nodes[cell[0]] = "key";
      nodes[cell[1]] = "key";
      if (k !== null) nodes[k] = "pivot";
    }
    const via: [number, number][] = [];
    if (k !== null) for (let x = 0; x < n; x++) via.push([x, k], [k, x]);
    r.snap({
      narration,
      marker,
      graph: {
        ...graphData(g, true, { nodes }),
        matrix: { label: "matrix", cells: dist.map((row) => row.map((d) => (d === INF ? null : d))), hot: cell ? [cell] : [], via },
      },
      vars: { k, i: cell ? cell[0] : null, j: cell ? cell[1] : null },
    });
  };

  snap(say("floyd-warshall.ready", { n, m: g.edges.length }), null);
  for (k = 0; k < n; k++) {
    snap(say("floyd-warshall.pivot", { k }), "pivot");
    for (let i = 0; i < n; i++) {
      for (let j = 0; j < n; j++) {
        r.comparisons++;
        const d = dist[i]![k]! + dist[k]![j]!;
        if (d < dist[i]![j]!) {
          const old = dist[i]![j]!;
          dist[i]![j] = d;
          r.writes++;
          snap(old === INF ? say("floyd-warshall.improve_new", { i, j, k, d }) : say("floyd-warshall.improve", { i, j, k, d, old }), "improve", [i, j]);
        }
      }
    }
  }
  k = null;
  snap(say("floyd-warshall.done", { checks: r.comparisons, improved: r.writes }), null);
  r.steps[r.steps.length - 1]!.result = dist.flat();
  return r.steps;
}

export const floydWarshall: TracerDef = {
  id: "floyd-warshall",
  subject: "graphs",
  scene: "graph",
  limits: { min_len: 5, max_len: 47, min: 0, max: 9 },
  input: { kind: "codec", codec },
  presets: {
    shortcuts: () => encodeGraph(5, true, [[0, 1, 4], [1, 2, 3], [0, 2, 9], [2, 3, 1], [3, 0, 2], [1, 4, 7], [3, 4, 2]]),
    random: (next) => randomGraph(next, { nMin: 4, nMax: MAX_FW_NODES, directed: true, weighted: true, density: 1.6, connected: true }),
    "one-way-ring": () => encodeGraph(4, true, [[0, 1, 1], [1, 2, 1], [2, 3, 1], [3, 0, 1]]),
  },
  markers: MARKERS,
  narrationKeys: KEYS,
  counters: [
    { slot: "comparisons", label: "comparisons" },
    { slot: "writes", label: "relaxed" },
  ],
  legend: ["pivot", "key"],
  roleLabels: { pivot: "through_node", key: "endpoints" },
  vars: ["k", "i", "j"],
  trace,
};
