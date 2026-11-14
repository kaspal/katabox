// Topological sort, Kahn's algorithm. Mirrors content/learn/graphs/topological-sort/sample.{py,ts}:
// count each node's incoming edges; nodes with none can go first. Taking a node removes
// its outgoing edges, which may free more nodes. Output: the order; if the graph has a
// cycle, the nodes on it never reach zero and the order comes out short.
import { adjacency, decodeGraph, encodeGraph, graphCodec, graphData, randomGraph } from "../engine/graph";
import { Recorder } from "../engine/recorder";
import type { NarrationKey, NarrationParams, Role, Step, TracerDef } from "../engine/types";

type Key = NarrationKey<"topological-sort">;
const KEYS = [
  "topological-sort.ready",
  "topological-sort.take",
  "topological-sort.lower",
  "topological-sort.ready_node",
  "topological-sort.done",
  "topological-sort.stuck",
] as const satisfies readonly Key[];
const say = (key: (typeof KEYS)[number], params?: NarrationParams<number>) => ({ key, params });

export const MARKERS = ["take", "lower", "ready"] as const;
const codec = graphCodec({ directed: true, weighted: false });

function trace(input: readonly number[]): Step[] {
  const g = decodeGraph(input);
  const adj = adjacency(g);
  const r = new Recorder([]);
  const indegree = new Array<number>(g.n).fill(0);
  for (const { v } of g.edges) indegree[v]!++;
  const queue: number[] = [];
  for (let u = 0; u < g.n; u++) if (indegree[u] === 0) queue.push(u);
  let head = 0;
  const order: number[] = [];
  const used: Record<number, Role> = {};
  const snap = (narration: ReturnType<typeof say>, marker: string | null, u: number | null, look?: { v: number; e: number }) => {
    const nodes: Record<number, Role> = {};
    for (const v of order) nodes[v] = "sorted";
    for (const q of queue.slice(head)) nodes[q] = "pivot";
    if (u !== null) nodes[u] = "key";
    const edges: Record<number, Role> = { ...used };
    if (look) {
      edges[look.e] = "compare";
      nodes[look.v] = indegree[look.v] === 0 ? "swap" : "compare";
    }
    r.snap({
      narration,
      marker,
      graph: graphData(g, false, { nodes, edges, notes: Object.fromEntries(indegree.map((d, v) => [v, `in ${d}`])) }),
      slots: { layout: "row", label: "queue", cells: queue.slice(head).map((v) => ({ id: v, value: v })) },
      aux: { label: "order", cells: order.map((v) => ({ id: v, value: v })), roles: {} },
      vars: { u, taken: order.length },
    });
  };

  snap(say("topological-sort.ready", { n: g.n, free: queue.length }), null, null);
  while (head < queue.length) {
    const u = queue[head++]!;
    order.push(u);
    r.writes++;
    snap(say("topological-sort.take", { u }), "take", u);
    for (const { v, e } of adj[u]!) {
      indegree[v]!--;
      used[e] = "out";
      r.comparisons++;
      snap(say("topological-sort.lower", { u, v, d: indegree[v]! }), "lower", u, { v, e });
      if (indegree[v] === 0) {
        queue.push(v);
        snap(say("topological-sort.ready_node", { v }), "ready", u);
      }
    }
  }
  snap(order.length === g.n ? say("topological-sort.done", { n: g.n }) : say("topological-sort.stuck", { left: g.n - order.length }), null, null);
  r.steps[r.steps.length - 1]!.result = order;
  return r.steps;
}

export const topologicalSort: TracerDef = {
  id: "topological-sort",
  subject: "graphs",
  scene: "graph",
  limits: { min_len: 5, max_len: 62, min: 0, max: 12 },
  input: { kind: "codec", codec },
  presets: {
    dag: () => encodeGraph(7, true, [[0, 2], [1, 2], [1, 3], [2, 4], [3, 4], [4, 5], [3, 6]]),
    random: (next) => randomGraph(next, { nMin: 5, nMax: 9, directed: true, weighted: false, density: 1.3, dag: true }),
    "with-a-cycle": () => encodeGraph(6, true, [[0, 1], [1, 2], [2, 3], [3, 1], [0, 4], [4, 5]]),
  },
  markers: MARKERS,
  narrationKeys: KEYS,
  counters: [
    { slot: "writes", label: "taken" },
    { slot: "comparisons", label: "edges_removed" },
  ],
  legend: ["key", "pivot", "compare", "swap", "sorted"],
  roleLabels: { key: "current", pivot: "in_queue", compare: "lowered", swap: "freed", sorted: "placed" },
  vars: ["u", "taken"],
  trace,
};
