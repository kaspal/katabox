// Kruskal's minimum spanning forest. Mirrors content/learn/graphs/kruskal/sample.{py,ts}:
// sort the edges by weight (ties: by u, then v); take an edge when its ends are in
// different trees, which union-find (with path halving) answers; skip it otherwise, as it
// would close a cycle. Output: the total weight, then the edges taken (u, v). Under each
// node: r and its tree's root, so two nodes in one tree show the same note.
import { decodeGraph, encodeGraph, graphCodec, graphData, randomGraph } from "../engine/graph";
import { Recorder } from "../engine/recorder";
import type { NarrationKey, Role, Step, TracerDef } from "../engine/types";

type Key = NarrationKey<"kruskal">;
const KEYS = ["kruskal.ready", "kruskal.consider", "kruskal.skip", "kruskal.join", "kruskal.done"] as const satisfies readonly Key[];
const say = (key: (typeof KEYS)[number], params?: Record<string, number>) => ({ key, params });

export const MARKERS = ["consider", "skip", "join"] as const;
const codec = graphCodec({ directed: false, weighted: true, wMin: 1, wMax: 9 });

function trace(input: readonly number[]): Step[] {
  const g = decodeGraph(input);
  const r = new Recorder([]);
  const parent = Array.from({ length: g.n }, (_, k) => k);
  const find = (x: number) => {
    while (parent[x] !== x) {
      parent[x] = parent[parent[x]!]!;
      x = parent[x]!;
    }
    return x;
  };
  const root = (x: number) => {
    while (parent[x] !== x) x = parent[x]!;
    return x;
  };
  const order = g.edges.map((e, k) => ({ ...e, k })).sort((a, b) => a.w - b.w || a.u - b.u || a.v - b.v);
  const taken = new Set<number>();
  const skipped = new Set<number>();
  let total = 0;
  let at = -1;
  const snap = (narration: ReturnType<typeof say>, marker: string | null, focus?: { e: number; role: Role }) => {
    // Trees show as their taken edges; the note under a node names its tree's root.
    const nodes: Record<number, Role> = {};
    const edges: Record<number, Role> = {};
    for (const e of taken) edges[e] = "sorted";
    for (const e of skipped) edges[e] = "out";
    if (focus) {
      edges[focus.e] = focus.role;
      nodes[g.edges[focus.e]!.u] = "key";
      nodes[g.edges[focus.e]!.v] = "key";
    }
    r.snap({
      narration,
      marker,
      graph: graphData(g, true, { nodes, edges, notes: Object.fromEntries(Array.from({ length: g.n }, (_, v) => [v, `r${root(v)}`])) }),
      slots: {
        layout: "row",
        label: "sorted_edges",
        cells: order.map((e) => ({ id: e.k, value: e.w, text: `${e.u}-${e.v}` })),
        roles: Object.fromEntries(order.map((e, j) => [j, (j === at ? "key" : taken.has(e.k) ? "sorted" : skipped.has(e.k) ? "out" : "idle") as Role])),
      },
      vars: { total, taken: taken.size },
    });
  };

  snap(say("kruskal.ready", { n: g.n, m: g.edges.length }), null);
  for (const [j, e] of order.entries()) {
    at = j;
    const ru = find(e.u);
    const rv = find(e.v);
    r.comparisons++;
    snap(say("kruskal.consider", { u: e.u, v: e.v, w: e.w }), "consider", { e: e.k, role: "key" });
    if (ru === rv) {
      skipped.add(e.k);
      snap(say("kruskal.skip", { u: e.u, v: e.v, root: ru }), "skip", { e: e.k, role: "out" });
      continue;
    }
    parent[ru] = rv;
    total += e.w;
    taken.add(e.k);
    r.writes++;
    snap(say("kruskal.join", { u: e.u, v: e.v, w: e.w, total }), "join", { e: e.k, role: "swap" });
  }
  at = -1;
  snap(say("kruskal.done", { total, taken: taken.size, n: g.n }), null);
  r.steps[r.steps.length - 1]!.result = [total, ...order.filter((e) => taken.has(e.k)).flatMap((e) => [e.u, e.v])];
  return r.steps;
}

export const kruskal: TracerDef = {
  id: "kruskal",
  subject: "graphs",
  scene: "graph",
  limits: { min_len: 5, max_len: 62, min: 0, max: 12 },
  input: { kind: "codec", codec },
  presets: {
    "cheap-ring": () => encodeGraph(6, false, [[0, 1, 4], [1, 2, 2], [2, 3, 5], [3, 4, 1], [4, 5, 3], [5, 0, 2], [0, 3, 8], [1, 4, 6]]),
    random: (next) => randomGraph(next, { nMin: 5, nMax: 9, directed: false, weighted: true, density: 1.6, connected: true }),
    "equal-weights": () => encodeGraph(5, false, [[0, 1, 2], [1, 2, 2], [2, 0, 2], [2, 3, 2], [3, 4, 2], [4, 2, 2]]),
  },
  markers: MARKERS,
  narrationKeys: KEYS,
  counters: [
    { slot: "comparisons", label: "edges_checked" },
    { slot: "writes", label: "joined" },
  ],
  legend: ["key", "swap", "sorted", "out"],
  roleLabels: { key: "current", swap: "joined", sorted: "in_tree", out: "skipped" },
  vars: ["total", "taken"],
  trace,
};
