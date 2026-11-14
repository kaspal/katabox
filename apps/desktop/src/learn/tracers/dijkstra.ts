// Dijkstra's shortest paths from node 0. Mirrors content/learn/graphs/dijkstra/sample.{py,ts}:
// a priority queue hands out (distance, node) pairs, smallest first (ties: smaller node);
// popping a node whose entry is older than its best distance is skipped ("stale"); every
// edge out of a popped node may lower a neighbour's distance and push a new entry.
// Weights are never negative. Output: each node's distance (-1 when unreachable).
import { adjacency, decodeGraph, encodeGraph, graphCodec, graphData, randomGraph } from "../engine/graph";
import { Recorder } from "../engine/recorder";
import type { NarrationKey, Role, Step, TracerDef } from "../engine/types";

type Key = NarrationKey<"dijkstra">;
const KEYS = [
  "dijkstra.ready",
  "dijkstra.pop",
  "dijkstra.stale",
  "dijkstra.relax",
  "dijkstra.no_better",
  "dijkstra.done",
] as const satisfies readonly Key[];
const say = (key: (typeof KEYS)[number], params?: Record<string, number>) => ({ key, params });

export const MARKERS = ["pop", "stale", "relax"] as const;
const codec = graphCodec({ directed: false, weighted: true, wMin: 1, wMax: 9 });

function trace(input: readonly number[]): Step[] {
  const g = decodeGraph(input);
  const adj = adjacency(g);
  const r = new Recorder([]);
  const dist: (number | null)[] = new Array(g.n).fill(null);
  const done = new Set<number>();
  const via: Record<number, number> = {};
  dist[0] = 0;
  let nextId = 0;
  let queue: { d: number; u: number; id: number }[] = [{ d: 0, u: 0, id: nextId++ }];
  const snap = (narration: ReturnType<typeof say>, marker: string | null, u: number | null, look?: { v: number; e: number; role: Role }) => {
    const nodes: Record<number, Role> = {};
    for (const q of queue) if (!done.has(q.u)) nodes[q.u] = "pivot";
    for (const v of done) nodes[v] = "sorted";
    if (u !== null) nodes[u] = "key";
    const edges: Record<number, Role> = Object.fromEntries(Object.values(via).map((e) => [e, "sorted" as Role]));
    if (look) {
      edges[look.e] = look.role;
      nodes[look.v] = look.role === "swap" ? "swap" : nodes[look.v] ?? "compare";
    }
    r.snap({
      narration,
      marker,
      graph: graphData(g, true, { nodes, edges, notes: Object.fromEntries(dist.map((d, v) => [v, d === null ? "∞" : String(d)])), marks: [...done] }),
      // The queue as (distance, node) cards: the value is the distance, the label the node.
      slots: { layout: "row", label: "pq", cells: queue.map((q) => ({ id: q.id, value: q.d, text: `${q.d}·${q.u}` })) },
      vars: { u, d: u === null ? null : dist[u]! },
    });
  };
  const sortQueue = () => {
    queue = [...queue].sort((x, y) => x.d - y.d || x.u - y.u);
  };

  snap(say("dijkstra.ready", { n: g.n }), null, null);
  while (queue.length) {
    const { d, u } = queue.shift()!;
    r.comparisons++;
    snap(say("dijkstra.pop", { u, d }), "pop", u);
    if (d > dist[u]!) {
      snap(say("dijkstra.stale", { u, d, best: dist[u]! }), "stale", u);
      continue;
    }
    done.add(u);
    for (const { v, w, e } of adj[u]!) {
      if (dist[v] === null || d + w < dist[v]!) {
        dist[v] = d + w;
        via[v] = e;
        queue.push({ d: d + w, u: v, id: nextId++ });
        sortQueue();
        r.writes++;
        snap(say("dijkstra.relax", { u, v, d: d + w, w }), "relax", u, { v, e, role: "swap" });
      } else {
        snap(say("dijkstra.no_better", { u, v, via: d + w, best: dist[v]! }), null, u, { v, e, role: "compare" });
      }
    }
  }
  const out = dist.map((x) => (x === null ? -1 : x));
  snap(say("dijkstra.done", { reached: dist.filter((x) => x !== null).length, n: g.n }), null, null);
  r.steps[r.steps.length - 1]!.result = out;
  return r.steps;
}

export const dijkstra: TracerDef = {
  id: "dijkstra",
  subject: "graphs",
  scene: "graph",
  limits: { min_len: 5, max_len: 62, min: 0, max: 12 },
  input: { kind: "codec", codec },
  presets: {
    "detour-wins": () => encodeGraph(6, false, [[0, 1, 7], [0, 2, 2], [2, 1, 3], [1, 3, 1], [2, 4, 8], [3, 4, 2], [4, 5, 3], [3, 5, 9]]),
    random: (next) => randomGraph(next, { nMin: 5, nMax: 9, directed: false, weighted: true, wMin: 1, wMax: 9, density: 1.5, connected: true }),
    "stale-entries": () => encodeGraph(5, false, [[0, 1, 9], [0, 2, 1], [2, 1, 1], [0, 3, 8], [2, 3, 2], [1, 4, 1], [3, 4, 5]]),
  },
  markers: MARKERS,
  narrationKeys: KEYS,
  counters: [
    { slot: "comparisons", label: "pops" },
    { slot: "writes", label: "relaxed" },
  ],
  legend: ["key", "pivot", "swap", "compare", "sorted"],
  roleLabels: { key: "current", pivot: "in_queue", swap: "relaxed", compare: "no_better", sorted: "settled" },
  vars: ["u", "d"],
  trace,
};
