// Strongly connected components, Kosaraju's algorithm. Mirrors content/learn/graphs/scc/sample.{py,ts}:
// pass 1, a DFS over every node, records the order in which nodes finish; pass 2 walks
// the reversed graph from the last finisher first: each walk stays inside one component
// and labels it. Output: each node's component number (in the order pass 2 finds them).
import { adjacency, decodeGraph, encodeGraph, graphCodec, graphData, randomGraph } from "../engine/graph";
import { Recorder } from "../engine/recorder";
import type { NarrationKey, NarrationParams, Role, Step, TracerDef } from "../engine/types";

type Key = NarrationKey<"scc">;
const KEYS = ["scc.ready", "scc.enter", "scc.finish", "scc.reverse", "scc.start", "scc.assign", "scc.done"] as const satisfies readonly Key[];
const say = (key: (typeof KEYS)[number], params?: NarrationParams<number>) => ({ key, params });

export const MARKERS = ["finish", "assign"] as const;
const codec = graphCodec({ directed: true, weighted: false });
/** Pass 2 colours, none of them used by pass 1; the note #c is the component itself. */
const PALETTE: Role[] = ["sorted", "swap", "compare"];

function trace(input: readonly number[]): Step[] {
  const g = decodeGraph(input);
  const adj = adjacency(g);
  const r = new Recorder([]);
  const seen = new Array<boolean>(g.n).fill(false);
  const finished: number[] = [];
  const label = new Array<number>(g.n).fill(-1);
  let pass = 1;
  let current: number | null = null;
  const snap = (narration: ReturnType<typeof say>, marker: string | null, extra: Record<number, Role> = {}) => {
    const nodes: Record<number, Role> = {};
    if (pass === 1) {
      seen.forEach((s, v) => {
        if (s) nodes[v] = "pivot";
      });
      for (const v of finished) nodes[v] = "out";
    } else {
      label.forEach((c, v) => {
        if (c >= 0) nodes[v] = PALETTE[c % PALETTE.length]!;
      });
    }
    if (current !== null) nodes[current] = "key";
    Object.assign(nodes, extra);
    const edges: Record<number, Role> = {};
    if (pass === 2) {
      g.edges.forEach((e, k) => {
        if (label[e.u]! >= 0 && label[e.u] === label[e.v]) edges[k] = PALETTE[label[e.u]! % PALETTE.length]!;
      });
    }
    const notes = Object.fromEntries(
      Array.from({ length: g.n }, (_, v) => [v, pass === 2 ? (label[v]! >= 0 ? `#${label[v]}` : "") : finished.includes(v) ? `f${finished.indexOf(v) + 1}` : ""]).filter(([, s]) => s !== ""),
    );
    r.snap({
      narration,
      marker,
      // Pass 2 walks the reversed graph: draw it reversed.
      graph: graphData(pass === 2 ? { ...g, edges: g.edges.map((e) => ({ ...e, u: e.v, v: e.u })) } : g, false, { nodes, edges, notes }),
      aux: { label: "order", cells: finished.map((v) => ({ id: v, value: v })), roles: {} },
      vars: { pass, components: Math.max(0, ...label) + (label.some((c) => c >= 0) ? 1 : 0) },
    });
  };
  const first = (u: number) => {
    seen[u] = true;
    current = u;
    r.comparisons++;
    snap(say("scc.enter", { u }), null);
    for (const { v } of adj[u]!) {
      if (!seen[v]) {
        first(v);
        current = u;
      }
    }
    finished.push(u);
    snap(say("scc.finish", { u, k: finished.length }), "finish");
  };
  const back: number[][] = Array.from({ length: g.n }, () => []);
  const second = (u: number, c: number) => {
    label[u] = c;
    current = u;
    r.writes++;
    snap(say("scc.assign", { u, c }), "assign");
    for (const v of back[u]!) if (label[v] === -1) second(v, c);
  };

  snap(say("scc.ready", { n: g.n }), null);
  for (let u = 0; u < g.n; u++) if (!seen[u]) first(u);
  for (let u = 0; u < g.n; u++) for (const { v } of adj[u]!) back[v]!.push(u);
  pass = 2;
  current = null;
  snap(say("scc.reverse"), null);
  let count = 0;
  for (const u of [...finished].reverse()) {
    if (label[u] === -1) {
      current = null;
      snap(say("scc.start", { u, c: count }), null, { [u]: "key" });
      second(u, count);
      count++;
    }
  }
  current = null;
  snap(say("scc.done", { num: count, n: g.n }), null);
  r.steps[r.steps.length - 1]!.result = label;
  return r.steps;
}

export const scc: TracerDef = {
  id: "scc",
  subject: "graphs",
  scene: "graph",
  limits: { min_len: 5, max_len: 62, min: 0, max: 12 },
  input: { kind: "codec", codec },
  presets: {
    "three-loops": () => encodeGraph(8, true, [[0, 1], [1, 2], [2, 0], [2, 3], [3, 4], [4, 5], [5, 3], [5, 6], [6, 7], [7, 6]]),
    random: (next) => randomGraph(next, { nMin: 5, nMax: 9, directed: true, weighted: false, density: 1.6 }),
    "one-way-chain": () => encodeGraph(5, true, [[0, 1], [1, 2], [2, 3], [3, 4]]),
  },
  markers: MARKERS,
  narrationKeys: KEYS,
  counters: [
    { slot: "comparisons", label: "visits" },
    { slot: "writes", label: "labelled" },
  ],
  legend: ["key", "pivot", "out", "sorted", "swap", "compare"],
  roleLabels: { key: "current", pivot: "on_the_path", out: "finished", sorted: "scc_a", swap: "scc_b", compare: "scc_c" },
  vars: ["pass", "components"],
  trace,
};
