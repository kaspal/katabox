// 0-1 BFS from node 0. Mirrors content/learn/graphs/zero-one-bfs/sample.{py,ts}: edges
// weigh 0 or 1; a deque replaces Dijkstra's priority queue. Reaching a node through a
// 0-edge puts it at the front (same distance), through a 1-edge at the back (one more).
// A node can be improved and queued again. Output: each node's distance (-1 unreachable).
import { adjacency, decodeGraph, encodeGraph, graphCodec, graphData, randomGraph } from "../engine/graph";
import { Recorder } from "../engine/recorder";
import type { NarrationKey, Role, Step, TracerDef } from "../engine/types";

type Key = NarrationKey<"zero-one-bfs">;
const KEYS = ["zero-one-bfs.ready", "zero-one-bfs.pop", "zero-one-bfs.front", "zero-one-bfs.back", "zero-one-bfs.no_better", "zero-one-bfs.done"] as const satisfies readonly Key[];
const say = (key: (typeof KEYS)[number], params?: Record<string, number>) => ({ key, params });

export const MARKERS = ["pop", "front", "back"] as const;
const codec = graphCodec({ directed: false, weighted: true, wMin: 0, wMax: 1 });

function trace(input: readonly number[]): Step[] {
  const g = decodeGraph(input);
  const adj = adjacency(g);
  const r = new Recorder([]);
  const dist = new Array<number>(g.n).fill(-1);
  dist[0] = 0;
  let nextId = 0;
  let deque: { u: number; id: number }[] = [{ u: 0, id: nextId++ }];
  const snap = (narration: ReturnType<typeof say>, marker: string | null, u: number | null, look?: { v: number; e: number; role: Role }, end?: "front" | "back") => {
    const nodes: Record<number, Role> = {};
    dist.forEach((d, v) => {
      if (d >= 0) nodes[v] = "sorted";
    });
    for (const q of deque) nodes[q.u] = "pivot";
    if (u !== null) nodes[u] = "key";
    const edges: Record<number, Role> = {};
    if (look) {
      edges[look.e] = look.role;
      nodes[look.v] = look.role;
    }
    const roles: Record<number, Role> = {};
    if (end === "front") roles[0] = "swap";
    if (end === "back") roles[deque.length - 1] = "swap";
    r.snap({
      narration,
      marker,
      graph: graphData(g, true, { nodes, edges, notes: Object.fromEntries(dist.map((d, v) => [v, d >= 0 ? String(d) : "∞"])) }),
      slots: { layout: "row", label: "deque", cells: deque.map((q) => ({ id: q.id, value: q.u })), roles },
      vars: { u, d: u === null ? null : dist[u]! },
    });
  };

  snap(say("zero-one-bfs.ready", { n: g.n }), null, null);
  while (deque.length) {
    const u = deque[0]!.u;
    deque = deque.slice(1);
    r.comparisons++;
    snap(say("zero-one-bfs.pop", { u, d: dist[u]! }), "pop", u);
    for (const { v, w, e } of adj[u]!) {
      if (dist[v] === -1 || dist[u]! + w < dist[v]!) {
        dist[v] = dist[u]! + w;
        r.writes++;
        if (w === 0) {
          deque = [{ u: v, id: nextId++ }, ...deque];
          snap(say("zero-one-bfs.front", { u, v, d: dist[v]! }), "front", u, { v, e, role: "swap" }, "front");
        } else {
          deque = [...deque, { u: v, id: nextId++ }];
          snap(say("zero-one-bfs.back", { u, v, d: dist[v]! }), "back", u, { v, e, role: "swap" }, "back");
        }
      } else {
        snap(say("zero-one-bfs.no_better", { u, v, via: dist[u]! + w, best: dist[v]! }), null, u, { v, e, role: "compare" });
      }
    }
  }
  snap(say("zero-one-bfs.done", { reached: dist.filter((d) => d >= 0).length, n: g.n, pops: r.comparisons }), null, null);
  r.steps[r.steps.length - 1]!.result = dist;
  return r.steps;
}

export const zeroOneBfs: TracerDef = {
  id: "zero-one-bfs",
  subject: "graphs",
  scene: "graph",
  limits: { min_len: 5, max_len: 62, min: 0, max: 12 },
  input: { kind: "codec", codec },
  presets: {
    "free-shortcut": () => encodeGraph(6, false, [[0, 1, 1], [1, 2, 1], [2, 5, 1], [0, 3, 0], [3, 4, 0], [4, 5, 1], [3, 1, 1]]),
    random: (next) => randomGraph(next, { nMin: 5, nMax: 9, directed: false, weighted: true, wMin: 0, wMax: 1, density: 1.4, connected: true }),
    "all-free": () => encodeGraph(5, false, [[0, 1, 0], [1, 2, 0], [2, 3, 0], [3, 4, 1], [0, 4, 1]]),
  },
  markers: MARKERS,
  narrationKeys: KEYS,
  counters: [
    { slot: "comparisons", label: "pops" },
    { slot: "writes", label: "relaxed" },
  ],
  legend: ["key", "pivot", "swap", "compare", "sorted"],
  roleLabels: { key: "current", pivot: "in_queue", swap: "relaxed", compare: "no_better", sorted: "reached" },
  vars: ["u", "d"],
  trace,
};
