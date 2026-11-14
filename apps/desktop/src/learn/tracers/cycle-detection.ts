// Cycle detection in a directed graph with three colours. Mirrors
// content/learn/graphs/cycle-detection/sample.{py,ts}: white = not seen, grey = on the
// current DFS path, black = finished. An edge into a grey node goes back up the path:
// that is a cycle. An edge into a black node is harmless. Output: 1 if there is a cycle,
// else 0. Colours are also written under the nodes, so they never rely on colour alone.
import { adjacency, decodeGraph, encodeGraph, graphCodec, graphData, randomGraph } from "../engine/graph";
import { Recorder } from "../engine/recorder";
import type { NarrationKey, Role, Step, TracerDef } from "../engine/types";

type Key = NarrationKey<"cycle-detection">;
const KEYS = [
  "cycle-detection.ready",
  "cycle-detection.grey",
  "cycle-detection.black_edge",
  "cycle-detection.back_edge",
  "cycle-detection.black",
  "cycle-detection.cycle",
  "cycle-detection.no_cycle",
] as const satisfies readonly Key[];
const say = (key: (typeof KEYS)[number], params?: Record<string, number>) => ({ key, params });

export const MARKERS = ["grey", "back-edge", "black"] as const;
const codec = graphCodec({ directed: true, weighted: false });
const NOTE = ["@white", "@grey", "@black"] as const;

function trace(input: readonly number[]): Step[] {
  const g = decodeGraph(input);
  const adj = adjacency(g);
  const r = new Recorder([]);
  const color = new Array<number>(g.n).fill(0);
  const path: number[] = [];
  const pathEdges: number[] = [];
  const snap = (narration: ReturnType<typeof say>, marker: string | null, look?: { e: number; role: Role }, extra: Record<number, Role> = {}) => {
    const nodes: Record<number, Role> = {};
    color.forEach((c, v) => {
      if (c === 1) nodes[v] = "pivot";
      if (c === 2) nodes[v] = "sorted";
    });
    if (path.length) nodes[path[path.length - 1]!] = "key";
    Object.assign(nodes, extra);
    const edges: Record<number, Role> = Object.fromEntries(pathEdges.map((e) => [e, "pivot" as Role]));
    if (look) edges[look.e] = look.role;
    r.snap({
      narration,
      marker,
      graph: graphData(g, false, { nodes, edges, notes: Object.fromEntries(color.map((c, v) => [v, NOTE[c]!])) }),
      slots: { layout: "row", label: "stack", cells: path.map((v) => ({ id: v, value: v })) },
      vars: { u: path.length ? path[path.length - 1]! : null },
    });
  };
  const visit = (u: number): boolean => {
    color[u] = 1;
    path.push(u);
    r.comparisons++;
    snap(say("cycle-detection.grey", { u }), "grey");
    for (const { v, e } of adj[u]!) {
      if (color[v] === 1) {
        snap(say("cycle-detection.back_edge", { u, v }), "back-edge", { e, role: "swap" }, { [v]: "swap" });
        return true;
      }
      if (color[v] === 2) {
        snap(say("cycle-detection.black_edge", { u, v }), null, { e, role: "compare" });
        continue;
      }
      pathEdges.push(e);
      if (visit(v)) return true;
      pathEdges.pop();
    }
    color[u] = 2;
    path.pop();
    snap(say("cycle-detection.black", { u }), "black", undefined, { [u]: "sorted" });
    return false;
  };

  snap(say("cycle-detection.ready", { n: g.n }), null);
  let found = 0;
  for (let u = 0; u < g.n && !found; u++) if (color[u] === 0 && visit(u)) found = 1;
  snap(found ? say("cycle-detection.cycle") : say("cycle-detection.no_cycle", { n: g.n }), null);
  r.steps[r.steps.length - 1]!.result = [found];
  return r.steps;
}

export const cycleDetection: TracerDef = {
  id: "cycle-detection",
  subject: "graphs",
  scene: "graph",
  limits: { min_len: 5, max_len: 62, min: 0, max: 12 },
  input: { kind: "codec", codec },
  presets: {
    "hidden-cycle": () => encodeGraph(7, true, [[0, 1], [1, 2], [0, 3], [3, 4], [4, 5], [5, 3], [2, 6]]),
    random: (next) => randomGraph(next, { nMin: 5, nMax: 8, directed: true, weighted: false, density: 1.2 }),
    dag: () => encodeGraph(7, true, [[0, 1], [0, 2], [1, 3], [2, 3], [3, 4], [2, 5], [5, 4], [4, 6]]),
  },
  markers: MARKERS,
  narrationKeys: KEYS,
  counters: [{ slot: "comparisons", label: "visits" }],
  legend: ["key", "pivot", "sorted", "swap", "compare"],
  roleLabels: { key: "current", pivot: "grey", sorted: "black", swap: "back_edge", compare: "harmless_edge" },
  vars: ["u"],
  trace,
};
