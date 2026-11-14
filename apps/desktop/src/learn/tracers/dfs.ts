// Depth-first search from node 0. Mirrors content/learn/graphs/dfs/sample.{py,ts}: enter a
// node, go into its first unseen neighbour, and only back up when a node has no unseen
// neighbours left. The call stack is the path from 0 to the current node. Output: the
// order nodes were entered, then the order they were finished.
import { adjacency, decodeGraph, encodeGraph, graphCodec, graphData, randomGraph } from "../engine/graph";
import { Recorder } from "../engine/recorder";
import type { NarrationKey, NarrationParams, Role, Step, TracerDef } from "../engine/types";

type Key = NarrationKey<"dfs">;
const KEYS = ["dfs.ready", "dfs.enter", "dfs.go", "dfs.seen", "dfs.leave", "dfs.done"] as const satisfies readonly Key[];
const say = (key: (typeof KEYS)[number], params?: NarrationParams<number>) => ({ key, params });

export const MARKERS = ["enter", "go", "leave"] as const;
const codec = graphCodec({ directed: false, weighted: false });

function trace(input: readonly number[]): Step[] {
  const g = decodeGraph(input);
  const adj = adjacency(g);
  const r = new Recorder([]);
  const seen = new Array<boolean>(g.n).fill(false);
  const order: number[] = [];
  const finished: number[] = [];
  const path: number[] = [];
  const tree: Record<number, Role> = {};
  const snap = (narration: ReturnType<typeof say>, marker: string | null, look?: { v: number; e: number; role: Role }) => {
    const nodes: Record<number, Role> = {};
    for (const v of order) nodes[v] = "pivot";
    for (const v of finished) nodes[v] = "sorted";
    if (path.length) nodes[path[path.length - 1]!] = "key";
    const edges: Record<number, Role> = { ...tree };
    if (look) {
      edges[look.e] = look.role;
      if (look.role === "compare") nodes[look.v] = nodes[look.v] ?? "compare";
    }
    const notes = Object.fromEntries(order.map((v, k) => [v, `${k + 1}`]));
    r.snap({
      narration,
      marker,
      graph: graphData(g, false, { nodes, edges, notes }),
      slots: { layout: "row", label: "stack", cells: path.map((v) => ({ id: v, value: v })), roles: path.length ? { [path.length - 1]: "key" } : {} },
      aux: { label: "order", cells: finished.map((v) => ({ id: v, value: v })), roles: {} },
      vars: { u: path.length ? path[path.length - 1]! : null, depth: path.length },
    });
  };
  const visit = (u: number) => {
    seen[u] = true;
    order.push(u);
    path.push(u);
    r.comparisons++;
    snap(say("dfs.enter", { u, k: order.length }), "enter");
    for (const { v, e } of adj[u]!) {
      if (!seen[v]) {
        tree[e] = "sorted";
        r.moves++;
        snap(say("dfs.go", { u, v }), "go", { v, e, role: "swap" });
        visit(v);
      } else {
        snap(say("dfs.seen", { u, v }), null, { v, e, role: "compare" });
      }
    }
    finished.push(u);
    snap(say("dfs.leave", { u }), "leave");
    path.pop();
  };

  snap(say("dfs.ready", { n: g.n }), null);
  visit(0);
  snap(say("dfs.done", { reached: order.length, n: g.n }), null);
  r.steps[r.steps.length - 1]!.result = [...order, ...finished];
  return r.steps;
}

export const dfs: TracerDef = {
  id: "dfs",
  subject: "graphs",
  scene: "graph",
  limits: { min_len: 5, max_len: 62, min: 0, max: 12 },
  input: { kind: "codec", codec },
  presets: {
    "deep-branch": () => encodeGraph(8, false, [[0, 1], [1, 2], [2, 3], [3, 4], [0, 5], [5, 6], [6, 7], [2, 6]]),
    random: (next) => randomGraph(next, { nMin: 5, nMax: 9, directed: false, weighted: false, density: 1.3, connected: true }),
    "grid-like": () => encodeGraph(9, false, [[0, 1], [1, 2], [0, 3], [1, 4], [2, 5], [3, 4], [4, 5], [3, 6], [4, 7], [5, 8], [6, 7], [7, 8]]),
  },
  markers: MARKERS,
  narrationKeys: KEYS,
  counters: [
    { slot: "comparisons", label: "visits" },
    { slot: "moves", label: "edges_followed" },
  ],
  legend: ["key", "pivot", "sorted", "compare"],
  roleLabels: { key: "current", pivot: "on_the_path", sorted: "finished", compare: "already_seen" },
  vars: ["u", "depth"],
  trace,
};
