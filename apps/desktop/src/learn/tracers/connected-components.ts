// Connected components by flood fill. Mirrors content/learn/graphs/connected-components/sample.{py,ts}:
// scan the nodes; an unlabelled node starts a new component, and a stack spreads its
// label to everything reachable from it. Output: each node's component number.
import { adjacency, decodeGraph, encodeGraph, graphCodec, graphData, randomGraph } from "../engine/graph";
import { Recorder } from "../engine/recorder";
import type { NarrationKey, NarrationParams, Role, Step, TracerDef } from "../engine/types";

type Key = NarrationKey<"connected-components">;
const KEYS = [
  "connected-components.ready",
  "connected-components.new",
  "connected-components.pop",
  "connected-components.fill",
  "connected-components.done",
] as const satisfies readonly Key[];
const say = (key: (typeof KEYS)[number], params?: NarrationParams<number>) => ({ key, params });

export const MARKERS = ["new", "fill"] as const;
const codec = graphCodec({ directed: false, weighted: false });
/** Component colours cycle through these roles; the number under each node is the label. */
const PALETTE: Role[] = ["sorted", "pivot", "swap", "compare"];

function trace(input: readonly number[]): Step[] {
  const g = decodeGraph(input);
  const adj = adjacency(g);
  const r = new Recorder([]);
  const label = new Array<number>(g.n).fill(-1);
  const stack: number[] = [];
  let count = 0;
  const snap = (narration: ReturnType<typeof say>, marker: string | null, u: number | null, e?: number) => {
    const nodes: Record<number, Role> = {};
    label.forEach((c, v) => {
      if (c >= 0) nodes[v] = PALETTE[c % PALETTE.length]!;
    });
    if (u !== null) nodes[u] = "key";
    const edges: Record<number, Role> = {};
    g.edges.forEach((ed, k) => {
      if (label[ed.u]! >= 0 && label[ed.u] === label[ed.v]) edges[k] = PALETTE[label[ed.u]! % PALETTE.length]!;
    });
    if (e !== undefined) edges[e] = "key";
    r.snap({
      narration,
      marker,
      graph: graphData(g, false, { nodes, edges, notes: Object.fromEntries(label.map((c, v) => [v, c >= 0 ? `#${c}` : ""]).filter(([, s]) => s !== "")) }),
      slots: { layout: "row", label: "stack", cells: stack.map((v) => ({ id: v, value: v })) },
      vars: { u, components: count },
    });
  };

  snap(say("connected-components.ready", { n: g.n }), null, null);
  for (let s = 0; s < g.n; s++) {
    if (label[s] !== -1) continue;
    label[s] = count;
    stack.push(s);
    r.writes++;
    snap(say("connected-components.new", { s, c: count }), "new", s);
    while (stack.length) {
      const u = stack.pop()!;
      snap(say("connected-components.pop", { u }), null, u);
      for (const { v, e } of adj[u]!) {
        r.comparisons++;
        if (label[v] === -1) {
          label[v] = count;
          stack.push(v);
          r.writes++;
          snap(say("connected-components.fill", { u, v, c: count }), "fill", u, e);
        }
      }
    }
    count++;
  }
  snap(say("connected-components.done", { num: count, n: g.n }), null, null);
  r.steps[r.steps.length - 1]!.result = label;
  return r.steps;
}

export const connectedComponents: TracerDef = {
  id: "connected-components",
  subject: "graphs",
  scene: "graph",
  limits: { min_len: 5, max_len: 62, min: 0, max: 12 },
  input: { kind: "codec", codec },
  presets: {
    islands: () => encodeGraph(10, false, [[0, 1], [1, 2], [3, 4], [4, 5], [5, 3], [6, 7], [8, 9], [7, 8]]),
    random: (next) => randomGraph(next, { nMin: 6, nMax: 11, directed: false, weighted: false, density: 0.7 }),
    "one-piece": () => encodeGraph(7, false, [[0, 1], [1, 2], [2, 3], [3, 4], [4, 5], [5, 6], [6, 0]]),
  },
  markers: MARKERS,
  narrationKeys: KEYS,
  counters: [
    { slot: "writes", label: "labelled" },
    { slot: "comparisons", label: "edges_followed" },
  ],
  legend: ["key", "sorted", "pivot", "swap", "compare"],
  roleLabels: { key: "current", sorted: "component_a", pivot: "component_b", swap: "component_c", compare: "component_d" },
  vars: ["u", "components"],
  trace,
};
