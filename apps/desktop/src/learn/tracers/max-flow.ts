// Maximum flow, Edmonds-Karp. Mirrors content/learn/graphs/max-flow/sample.{py,ts}: source 0,
// sink n − 1. Repeatedly find the shortest path (fewest edges) with spare capacity by BFS,
// scanning neighbours in node order, then push its bottleneck along it; pushing u → v
// also opens v → u by the same amount (flow can be taken back later). Stops when the
// sink is unreachable. Output: [the maximum flow]. Edge labels show flow / capacity.
import { decodeGraph, encodeGraph, graphCodec, graphData, randomGraph } from "../engine/graph";
import { Recorder } from "../engine/recorder";
import type { NarrationKey, Role, Step, TracerDef } from "../engine/types";

type Key = NarrationKey<"max-flow">;
const KEYS = ["max-flow.ready", "max-flow.search", "max-flow.reach", "max-flow.path", "max-flow.augment", "max-flow.done"] as const satisfies readonly Key[];
const say = (key: (typeof KEYS)[number], params?: Record<string, number>) => ({ key, params });

export const MARKERS = ["reach", "augment"] as const;
export const MAX_FLOW_NODES = 8;
const codec = graphCodec({ directed: true, weighted: true, wMin: 1, wMax: 9, maxNodes: MAX_FLOW_NODES, maxEdges: 14, antiparallel: false });

function trace(input: readonly number[]): Step[] {
  const g = decodeGraph(input);
  const n = g.n;
  const r = new Recorder([]);
  const cap = Array.from({ length: n }, () => new Array<number>(n).fill(0));
  for (const { u, v, w } of g.edges) cap[u]![v] = w;
  const edgeOf = new Map(g.edges.map((e, k) => [`${e.u}>${e.v}`, k]));
  const source = 0;
  const sink = n - 1;
  let total = 0;
  let parent = new Array<number>(n).fill(-1);
  const snap = (narration: ReturnType<typeof say>, marker: string | null, hot: { nodes?: number[]; edges?: number[]; role?: Role } = {}) => {
    const nodes: Record<number, Role> = { [source]: "pivot", [sink]: "pivot" };
    parent.forEach((p, v) => {
      if (p !== -1 && v !== source) nodes[v] = "sorted";
    });
    const edges: Record<number, Role> = {};
    g.edges.forEach((e, k) => {
      if (cap[e.u]![e.v] === 0) edges[k] = "out";
    });
    for (const v of hot.nodes ?? []) nodes[v] = hot.role ?? "key";
    for (const e of hot.edges ?? []) edges[e] = hot.role ?? "key";
    // flow on an original edge = capacity − residual.
    const labels = Object.fromEntries(g.edges.map((e, k) => [k, `${e.w - cap[e.u]![e.v]!}/${e.w}`]));
    r.snap({ narration, marker, graph: graphData(g, true, { nodes, edges, labels }), vars: { flow: total } });
  };

  snap(say("max-flow.ready", { n, source, sink }), null);
  for (;;) {
    parent = new Array<number>(n).fill(-1);
    parent[source] = source;
    snap(say("max-flow.search", { flow: total }), null, { nodes: [source], role: "key" });
    const queue = [source];
    while (queue.length && parent[sink] === -1) {
      const u = queue.shift()!;
      for (let v = 0; v < n; v++) {
        if (parent[v] === -1 && cap[u]![v]! > 0) {
          parent[v] = u;
          queue.push(v);
          r.comparisons++;
          const e = edgeOf.get(`${u}>${v}`) ?? edgeOf.get(`${v}>${u}`);
          snap(say("max-flow.reach", { u, v, spare: cap[u]![v]! }), "reach", { nodes: [v], edges: e === undefined ? [] : [e], role: "compare" });
        }
      }
    }
    if (parent[sink] === -1) break;
    let push = 999;
    const pathNodes: number[] = [sink];
    const pathEdges: number[] = [];
    for (let v = sink; v !== source; v = parent[v]!) {
      push = Math.min(push, cap[parent[v]!]![v]!);
      pathNodes.push(parent[v]!);
      const e = edgeOf.get(`${parent[v]}>${v}`) ?? edgeOf.get(`${v}>${parent[v]}`);
      if (e !== undefined) pathEdges.push(e);
    }
    snap(say("max-flow.path", { push, len: pathEdges.length }), null, { nodes: pathNodes, edges: pathEdges, role: "swap" });
    for (let v = sink; v !== source; v = parent[v]!) {
      cap[parent[v]!]![v]! -= push;
      cap[v]![parent[v]!]! += push;
      r.writes++;
      const e = edgeOf.get(`${parent[v]}>${v}`) ?? edgeOf.get(`${v}>${parent[v]}`);
      snap(say("max-flow.augment", { u: parent[v]!, v, push }), "augment", { nodes: [parent[v]!, v], edges: e === undefined ? [] : [e], role: "swap" });
    }
    total += push;
    r.moves++;
  }
  parent = new Array<number>(n).fill(-1);
  snap(say("max-flow.done", { flow: total, paths: r.moves }), null);
  r.steps[r.steps.length - 1]!.result = [total];
  return r.steps;
}

export const maxFlow: TracerDef = {
  id: "max-flow",
  subject: "graphs",
  scene: "graph",
  limits: { min_len: 5, max_len: 44, min: 0, max: 9 },
  input: { kind: "codec", codec },
  presets: {
    "take-it-back": () => encodeGraph(4, true, [[0, 1, 1], [0, 2, 1], [1, 2, 1], [1, 3, 1], [2, 3, 1]]),
    random: (next) => randomGraph(next, { nMin: 4, nMax: MAX_FLOW_NODES, directed: true, weighted: true, density: 1.5, connected: true, dag: true }),
    "two-lanes": () => encodeGraph(6, true, [[0, 1, 5], [0, 2, 4], [1, 3, 3], [2, 4, 6], [1, 4, 2], [3, 5, 4], [4, 5, 7]]),
  },
  markers: MARKERS,
  narrationKeys: KEYS,
  counters: [
    { slot: "comparisons", label: "visits" },
    { slot: "moves", label: "paths" },
    { slot: "writes", label: "writes" },
  ],
  legend: ["pivot", "key", "compare", "swap", "sorted", "out"],
  roleLabels: { pivot: "source_or_sink", key: "current", compare: "reached", swap: "augmenting", sorted: "visited", out: "full" },
  vars: ["flow"],
  trace,
};
