// Bridges and articulation points (cut vertices) by low-link. Mirrors
// content/learn/graphs/bridges-articulation/sample.{py,ts}: a DFS numbers nodes as it
// enters them (tin); low[u] is the smallest number u's subtree reaches through one back
// edge. A tree edge u-v is a bridge when low[v] > tin[u] (v's subtree cannot get back
// above v); u is a cut vertex when low[v] >= tin[u] for some child v (the root: when it
// has two or more children). Output: bridges as (smaller, larger) pairs, sorted, then
// -1, then the cut vertices in increasing order. Under each node: tin/low.
import { adjacency, decodeGraph, encodeGraph, graphCodec, graphData, randomGraph } from "../engine/graph";
import { Recorder } from "../engine/recorder";
import type { NarrationKey, NarrationParams, Role, Step, TracerDef } from "../engine/types";

type Key = NarrationKey<"bridges-articulation">;
const KEYS = [
  "bridges-articulation.ready",
  "bridges-articulation.enter",
  "bridges-articulation.back",
  "bridges-articulation.low",
  "bridges-articulation.bridge",
  "bridges-articulation.cut",
  "bridges-articulation.root_cut",
  "bridges-articulation.done",
] as const satisfies readonly Key[];
const say = (key: (typeof KEYS)[number], params?: NarrationParams<number>) => ({ key, params });

export const MARKERS = ["enter", "back", "low", "bridge", "cut", "root-cut"] as const;
const codec = graphCodec({ directed: false, weighted: false });

function trace(input: readonly number[]): Step[] {
  const g = decodeGraph(input);
  const adj = adjacency(g);
  const r = new Recorder([]);
  const tin = new Array<number>(g.n).fill(-1);
  const low = new Array<number>(g.n).fill(0);
  const bridges: [number, number][] = [];
  const bridgeEdges = new Set<number>();
  const treeEdges = new Set<number>();
  const cuts = new Set<number>();
  const path: number[] = [];
  let timer = 0;
  const snap = (narration: ReturnType<typeof say>, marker: string | null, focus?: { e?: number; role: Role; nodes: number[] }) => {
    const nodes: Record<number, Role> = {};
    tin.forEach((t, v) => {
      if (t >= 0) nodes[v] = "sorted";
    });
    for (const v of path) nodes[v] = "pivot";
    for (const v of cuts) nodes[v] = "swap";
    if (path.length) nodes[path[path.length - 1]!] = "key";
    const edges: Record<number, Role> = {};
    for (const e of treeEdges) edges[e] = "sorted";
    for (const e of bridgeEdges) edges[e] = "swap";
    if (focus) {
      if (focus.e !== undefined) edges[focus.e] = focus.role;
      for (const v of focus.nodes) nodes[v] = focus.role;
    }
    r.snap({
      narration,
      marker,
      graph: graphData(g, false, { nodes, edges, notes: Object.fromEntries(tin.map((t, v) => [v, t >= 0 ? `${t}/${low[v]}` : ""]).filter(([, s]) => s !== "")), marks: [...cuts] }),
      vars: { u: path.length ? path[path.length - 1]! : null, timer },
    });
  };
  const visit = (u: number, parent: number) => {
    tin[u] = low[u] = timer;
    timer++;
    path.push(u);
    r.comparisons++;
    snap(say("bridges-articulation.enter", { u, t: tin[u]! }), "enter");
    let children = 0;
    for (const { v, e } of adj[u]!) {
      if (v === parent) continue;
      if (tin[v] !== -1) {
        low[u] = Math.min(low[u]!, tin[v]!);
        snap(say("bridges-articulation.back", { u, v, t: tin[v]!, low: low[u]! }), "back", { e, role: "compare", nodes: [] });
      } else {
        treeEdges.add(e);
        visit(v, u);
        children++;
        low[u] = Math.min(low[u]!, low[v]!);
        snap(say("bridges-articulation.low", { u, v, low: low[u]! }), "low", { e, role: "key", nodes: [] });
        if (low[v]! > tin[u]!) {
          bridges.push([Math.min(u, v), Math.max(u, v)]);
          bridgeEdges.add(e);
          r.writes++;
          snap(say("bridges-articulation.bridge", { u, v, low: low[v]!, t: tin[u]! }), "bridge", { e, role: "swap", nodes: [] });
        }
        if (parent !== -1 && low[v]! >= tin[u]!) {
          cuts.add(u);
          snap(say("bridges-articulation.cut", { u, v, low: low[v]!, t: tin[u]! }), "cut", { role: "swap", nodes: [u] });
        }
      }
    }
    if (parent === -1 && children > 1) {
      cuts.add(u);
      snap(say("bridges-articulation.root_cut", { u, children }), "root-cut", { role: "swap", nodes: [u] });
    }
    path.pop();
  };

  snap(say("bridges-articulation.ready", { n: g.n }), null);
  for (let u = 0; u < g.n; u++) if (tin[u] === -1) visit(u, -1);
  const out = [...[...bridges].sort((a, b) => a[0] - b[0] || a[1] - b[1]).flat(), -1, ...[...cuts].sort((a, b) => a - b)];
  snap(say("bridges-articulation.done", { bridges: bridges.length, cuts: cuts.size }), null);
  r.steps[r.steps.length - 1]!.result = out;
  return r.steps;
}

export const bridgesArticulation: TracerDef = {
  id: "bridges-articulation",
  subject: "graphs",
  scene: "graph",
  limits: { min_len: 5, max_len: 62, min: 0, max: 12 },
  input: { kind: "codec", codec },
  presets: {
    "two-rings": () => encodeGraph(7, false, [[0, 1], [1, 2], [2, 0], [2, 3], [3, 4], [4, 5], [5, 3], [5, 6]]),
    random: (next) => randomGraph(next, { nMin: 5, nMax: 9, directed: false, weighted: false, density: 1.15, connected: true }),
    "a-tree": () => encodeGraph(6, false, [[0, 1], [0, 2], [1, 3], [1, 4], [2, 5]]),
  },
  markers: MARKERS,
  narrationKeys: KEYS,
  counters: [
    { slot: "comparisons", label: "visits" },
    { slot: "writes", label: "bridges_found" },
  ],
  legend: ["key", "pivot", "sorted", "compare", "swap"],
  roleLabels: { key: "current", pivot: "on_the_path", sorted: "visited", compare: "back_edge", swap: "critical" },
  vars: ["u", "timer"],
  trace,
};
