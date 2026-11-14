// Breadth-first search from node 0. Mirrors content/learn/graphs/bfs/sample.{py,ts}: a
// FIFO queue visits nodes in order of distance; the first time a node is seen, its
// distance is its parent's plus one. Output: each node's distance in edges (-1 when 0
// cannot reach it).
import { adjacency, decodeGraph, encodeGraph, graphCodec, graphData, randomGraph } from "../engine/graph";
import { Recorder } from "../engine/recorder";
import type { NarrationKey, Role, Step, TracerDef } from "../engine/types";

type Key = NarrationKey<"bfs">;
const KEYS = ["bfs.ready", "bfs.visit", "bfs.discover", "bfs.seen", "bfs.done"] as const satisfies readonly Key[];
const say = (key: (typeof KEYS)[number], params?: Record<string, number>) => ({ key, params });

export const MARKERS = ["visit", "discover"] as const;
const codec = graphCodec({ directed: false, weighted: false });

function trace(input: readonly number[]): Step[] {
  const g = decodeGraph(input);
  const adj = adjacency(g);
  const r = new Recorder([]);
  const dist = new Array<number>(g.n).fill(-1);
  const tree: Record<number, Role> = {};
  const queue: number[] = [0];
  let head = 0;
  dist[0] = 0;
  const snap = (narration: ReturnType<typeof say>, marker: string | null, u: number | null, look?: { v: number; e: number; fresh: boolean }) => {
    const nodes: Record<number, Role> = {};
    for (let k = 0; k < g.n; k++) if (dist[k]! >= 0) nodes[k] = "sorted";
    for (const q of queue.slice(head)) nodes[q] = "pivot";
    if (u !== null) nodes[u] = "key";
    const edges: Record<number, Role> = { ...tree };
    if (look) {
      nodes[look.v] = look.fresh ? "swap" : nodes[look.v]!;
      edges[look.e] = look.fresh ? "sorted" : "compare";
    }
    const notes = Object.fromEntries(dist.map((d, k) => [k, d >= 0 ? String(d) : ""]).filter(([, d]) => d !== ""));
    r.snap({
      narration,
      marker,
      graph: graphData(g, false, { nodes, edges, notes }),
      slots: { layout: "row", label: "queue", cells: queue.slice(head).map((v) => ({ id: v, value: v })) },
      vars: { u, dist: u === null ? null : dist[u]! },
    });
  };

  snap(say("bfs.ready", { n: g.n }), null, null);
  while (head < queue.length) {
    const u = queue[head++]!;
    r.comparisons++;
    snap(say("bfs.visit", { u, d: dist[u]! }), "visit", u);
    for (const { v, e } of adj[u]!) {
      if (dist[v] === -1) {
        dist[v] = dist[u]! + 1;
        queue.push(v);
        tree[e] = "sorted";
        r.writes++;
        snap(say("bfs.discover", { u, v, d: dist[v]! }), "discover", u, { v, e, fresh: true });
      } else {
        snap(say("bfs.seen", { v, d: dist[v]! }), null, u, { v, e, fresh: false });
      }
    }
  }
  snap(say("bfs.done", { reached: dist.filter((d) => d >= 0).length, n: g.n, far: Math.max(...dist) }), null, null);
  r.steps[r.steps.length - 1]!.result = dist;
  return r.steps;
}

export const bfs: TracerDef = {
  id: "bfs",
  subject: "graphs",
  scene: "graph",
  limits: { min_len: 5, max_len: 62, min: 0, max: 12 },
  input: { kind: "codec", codec },
  presets: {
    "grid-like": () => encodeGraph(9, false, [[0, 1], [1, 2], [0, 3], [1, 4], [2, 5], [3, 4], [4, 5], [3, 6], [4, 7], [5, 8], [6, 7], [7, 8]]),
    random: (next) => randomGraph(next, { nMin: 5, nMax: 9, directed: false, weighted: false, density: 1.3, connected: true }),
    "two-components": () => encodeGraph(8, false, [[0, 1], [0, 2], [1, 3], [2, 3], [4, 5], [5, 6], [6, 7], [7, 4]]),
  },
  markers: MARKERS,
  narrationKeys: KEYS,
  counters: [
    { slot: "comparisons", label: "visits" },
    { slot: "writes", label: "enqueued" },
  ],
  legend: ["key", "pivot", "swap", "sorted", "compare"],
  roleLabels: { key: "current", pivot: "in_queue", swap: "discovered", sorted: "visited", compare: "already_seen" },
  vars: ["u", "dist"],
  trace,
};
