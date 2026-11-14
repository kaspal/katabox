// Prim's minimum spanning tree from node 0. Mirrors content/learn/graphs/prim/sample.{py,ts}:
// grow one tree; a priority queue holds (weight, node, from) for every edge leaving it,
// smallest first; a popped node already in the tree is skipped. Output: the tree's total
// weight, then the nodes in the order they joined (only 0's component).
import { adjacency, decodeGraph, encodeGraph, graphCodec, graphData, randomGraph } from "../engine/graph";
import { Recorder } from "../engine/recorder";
import type { NarrationKey, NarrationParams, Role, Step, TracerDef } from "../engine/types";

type Key = NarrationKey<"prim">;
const KEYS = ["prim.ready", "prim.pop", "prim.skip", "prim.take", "prim.push", "prim.done"] as const satisfies readonly Key[];
const say = (key: (typeof KEYS)[number], params?: NarrationParams<number>) => ({ key, params });

export const MARKERS = ["pop", "skip", "take", "push"] as const;
const codec = graphCodec({ directed: false, weighted: true, wMin: 1, wMax: 9 });

type Entry = { w: number; v: number; u: number; e: number; id: number };

function trace(input: readonly number[]): Step[] {
  const g = decodeGraph(input);
  const adj = adjacency(g);
  const r = new Recorder([]);
  const inTree = new Array<boolean>(g.n).fill(false);
  const order: number[] = [];
  const treeEdges = new Set<number>();
  let total = 0;
  let nextId = 0;
  let queue: Entry[] = [{ w: 0, v: 0, u: 0, e: -1, id: nextId++ }];
  const snap = (narration: ReturnType<typeof say>, marker: string | null, focus?: { v: number; e: number; role: Role }) => {
    const nodes: Record<number, Role> = {};
    inTree.forEach((t, v) => {
      if (t) nodes[v] = "sorted";
    });
    const edges: Record<number, Role> = {};
    for (const q of queue) if (q.e >= 0) edges[q.e] = "pivot";
    for (const e of treeEdges) edges[e] = "sorted";
    if (focus) {
      nodes[focus.v] = focus.role;
      if (focus.e >= 0) edges[focus.e] = focus.role;
    }
    r.snap({
      narration,
      marker,
      graph: graphData(g, true, { nodes, edges, marks: order }),
      slots: { layout: "row", label: "pq", cells: queue.map((q) => ({ id: q.id, value: q.w, text: `${q.w}·${q.v}` })) },
      aux: { label: "order", cells: order.map((v) => ({ id: v, value: v })), roles: {} },
      vars: { total },
    });
  };
  const sortQueue = () => {
    queue = [...queue].sort((p, q) => p.w - q.w || p.v - q.v || p.u - q.u);
  };

  snap(say("prim.ready", { n: g.n }), null);
  while (queue.length) {
    const top = queue[0]!;
    queue = queue.slice(1);
    r.comparisons++;
    snap(say("prim.pop", { v: top.v, w: top.w }), "pop", { v: top.v, e: top.e, role: "key" });
    if (inTree[top.v]) {
      snap(say("prim.skip", { v: top.v }), "skip", { v: top.v, e: top.e, role: "out" });
      continue;
    }
    inTree[top.v] = true;
    order.push(top.v);
    if (top.e >= 0) treeEdges.add(top.e);
    total += top.w;
    r.writes++;
    snap(say("prim.take", { v: top.v, w: top.w, total }), "take", { v: top.v, e: top.e, role: "swap" });
    for (const { v: x, w, e } of adj[top.v]!) {
      if (!inTree[x]) {
        queue.push({ w, v: x, u: top.v, e, id: nextId++ });
        sortQueue();
        snap(say("prim.push", { u: top.v, v: x, w }), "push", { v: x, e, role: "compare" });
      }
    }
  }
  snap(say("prim.done", { total, size: order.length, n: g.n }), null);
  r.steps[r.steps.length - 1]!.result = [total, ...order];
  return r.steps;
}

export const prim: TracerDef = {
  id: "prim",
  subject: "graphs",
  scene: "graph",
  limits: { min_len: 5, max_len: 62, min: 0, max: 12 },
  input: { kind: "codec", codec },
  presets: {
    "cheap-ring": () => encodeGraph(6, false, [[0, 1, 4], [1, 2, 2], [2, 3, 5], [3, 4, 1], [4, 5, 3], [5, 0, 2], [0, 3, 8], [1, 4, 6]]),
    random: (next) => randomGraph(next, { nMin: 5, nMax: 9, directed: false, weighted: true, density: 1.6, connected: true }),
    "two-islands": () => encodeGraph(6, false, [[0, 1, 3], [1, 2, 1], [0, 2, 2], [3, 4, 1], [4, 5, 2]]),
  },
  markers: MARKERS,
  narrationKeys: KEYS,
  counters: [
    { slot: "comparisons", label: "pops" },
    { slot: "writes", label: "joined" },
  ],
  legend: ["key", "compare", "pivot", "swap", "sorted", "out"],
  roleLabels: { key: "current", compare: "pushed", pivot: "in_queue", swap: "joined", sorted: "in_tree", out: "skipped" },
  vars: ["total"],
  trace,
};
