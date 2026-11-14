// Eulerian path, Hierholzer's algorithm. Mirrors content/learn/graphs/eulerian-path/sample.{py,ts}:
// a path using every edge exactly once exists only with 0 or 2 odd-degree nodes; start at
// an odd node (else at the first edge's u). Walk unused edges, pushing nodes on a stack;
// when a node has none left, pop it onto the path. The path comes out reversed. If some
// edge was never reached (two pieces), there is no path. Output: the nodes in order, or [-1].
import { decodeGraph, encodeGraph, graphCodec, graphData } from "../engine/graph";
import { int } from "../engine/inputs";
import { Recorder } from "../engine/recorder";
import type { NarrationKey, Role, Step, TracerDef } from "../engine/types";

type Key = NarrationKey<"eulerian-path">;
const KEYS = [
  "eulerian-path.ready",
  "eulerian-path.odd",
  "eulerian-path.walk",
  "eulerian-path.back",
  "eulerian-path.done",
  "eulerian-path.missed",
] as const satisfies readonly Key[];
const say = (key: (typeof KEYS)[number], params?: Record<string, number>) => ({ key, params });

export const MARKERS = ["walk", "back"] as const;
const codec = graphCodec({ directed: false, weighted: false, maxNodes: 8, maxEdges: 14 });

function trace(input: readonly number[]): Step[] {
  const g = decodeGraph(input);
  const r = new Recorder([]);
  const adj: { v: number; k: number }[][] = Array.from({ length: g.n }, () => []);
  g.edges.forEach(({ u, v }, k) => {
    adj[u]!.push({ v, k });
    adj[v]!.push({ v: u, k });
  });
  const odd = adj.flatMap((l, u) => (l.length % 2 === 1 ? [u] : []));
  const used = new Array<boolean>(g.edges.length).fill(false);
  const stack: number[] = [];
  const path: number[] = [];
  const snap = (narration: ReturnType<typeof say>, marker: string | null, e?: number) => {
    const nodes: Record<number, Role> = {};
    for (const u of odd) nodes[u] = "pivot";
    for (const u of path) nodes[u] = "sorted";
    if (stack.length) nodes[stack[stack.length - 1]!] = "key";
    const edges: Record<number, Role> = {};
    used.forEach((x, k) => {
      if (x) edges[k] = "sorted";
    });
    if (e !== undefined) edges[e] = "swap";
    r.snap({
      narration,
      marker,
      graph: graphData(g, false, { nodes, edges, notes: Object.fromEntries(adj.map((l, u) => [u, `deg ${l.length}`])) }),
      slots: { layout: "row", label: "stack", cells: stack.map((v, j) => ({ id: j, value: v })) },
      aux: { label: "path", cells: path.map((v, j) => ({ id: j, value: v })), roles: {} },
      vars: { used: used.filter(Boolean).length, edges: g.edges.length },
    });
  };

  snap(say("eulerian-path.ready", { n: g.n, m: g.edges.length, odd: odd.length }), null);
  if (odd.length !== 0 && odd.length !== 2) {
    snap(say("eulerian-path.odd", { odd: odd.length }), null);
    r.steps[r.steps.length - 1]!.result = [-1];
    return r.steps;
  }
  const next = new Array<number>(g.n).fill(0);
  stack.push(odd.length ? odd[0]! : g.edges[0]!.u);
  while (stack.length) {
    const u = stack[stack.length - 1]!;
    while (next[u]! < adj[u]!.length && used[adj[u]![next[u]!]!.k]) next[u]!++;
    if (next[u]! < adj[u]!.length) {
      const { v, k } = adj[u]![next[u]!]!;
      used[k] = true;
      stack.push(v);
      r.moves++;
      snap(say("eulerian-path.walk", { u, v }), "walk", k);
    } else {
      path.push(stack.pop()!);
      r.writes++;
      snap(say("eulerian-path.back", { u }), "back");
    }
  }
  if (path.length !== g.edges.length + 1) {
    snap(say("eulerian-path.missed", { left: g.edges.length - used.filter(Boolean).length }), null);
    r.steps[r.steps.length - 1]!.result = [-1];
    return r.steps;
  }
  const out = [...path].reverse();
  snap(say("eulerian-path.done", { m: g.edges.length, start: out[0]!, end: out[out.length - 1]! }), null);
  r.steps[r.steps.length - 1]!.result = out;
  return r.steps;
}

export const eulerianPath: TracerDef = {
  id: "eulerian-path",
  subject: "graphs",
  scene: "graph",
  limits: { min_len: 5, max_len: 44, min: 0, max: 8 },
  input: { kind: "codec", codec },
  presets: {
    "house-of-cards": () => encodeGraph(5, false, [[0, 1], [1, 2], [2, 3], [3, 0], [0, 2], [1, 4], [4, 2]]),
    // A random trail (no edge twice), listed in shuffled order: a path always exists.
    random: (next) => {
      const n = int(next, 4, 7);
      const seen = new Set<string>();
      const edges: [number, number][] = [];
      let at = int(next, 0, n - 1);
      for (let tries = 0; edges.length < n + 3 && tries < 60; tries++) {
        const to = int(next, 0, n - 1);
        const key = `${Math.min(at, to)}-${Math.max(at, to)}`;
        if (to === at || seen.has(key)) continue;
        seen.add(key);
        edges.push([at, to]);
        at = to;
      }
      for (let k = edges.length - 1; k > 0; k--) {
        const j = int(next, 0, k);
        [edges[k], edges[j]] = [edges[j]!, edges[k]!];
      }
      return encodeGraph(n, false, edges);
    },
    "detour-loop": () => encodeGraph(6, false, [[0, 1], [1, 2], [2, 3], [1, 4], [4, 5], [5, 1]]),
    "too-many-odd": () => encodeGraph(4, false, [[0, 1], [0, 2], [0, 3]]),
  },
  markers: MARKERS,
  narrationKeys: KEYS,
  counters: [
    { slot: "moves", label: "edges_followed" },
    { slot: "writes", label: "placed" },
  ],
  legend: ["key", "pivot", "swap", "sorted"],
  roleLabels: { key: "current", pivot: "odd_degree", swap: "just_walked", sorted: "used_or_placed" },
  vars: ["used", "edges"],
  trace,
};
