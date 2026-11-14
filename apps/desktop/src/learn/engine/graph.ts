// Graph inputs for the Learn topics (ADR 0029, Phases 4-5). A graph is stored as one flat
// integer list, `[n, directed, u, v, w, …]` (unweighted edges store w = 1), and typed in
// "your own" as a node count plus an edge list such as `0 1 4, 1 2 3` (the minimal graph
// editor: add or remove an edge by editing the list, nodes by changing n). The Learn
// driver's `kb_graph` / `kb_edges` decode the same list in the same order.
import { fail } from "./codecs";
import { int, tokens, type InputError } from "./inputs";
import type { FieldKey, GraphData, GraphEdge, GraphNode, InputCodec, Role } from "./types";

export type Edge = { u: number; v: number; w: number };
export type Graph = { n: number; directed: boolean; edges: Edge[] };
export type GraphOptions = {
  /** Fixed by the topic, or chosen in "your own" (a 0 / 1 field). */
  directed: boolean | "choose";
  weighted: boolean;
  wMin?: number;
  wMax?: number;
  minNodes?: number;
  maxNodes?: number;
  maxEdges?: number;
};

export const MAX_NODES = 12;
export const MAX_EDGES = 20;

export function decodeGraph(input: readonly number[]): Graph {
  const edges: Edge[] = [];
  for (let k = 2; k + 2 < input.length; k += 3) edges.push({ u: input[k]!, v: input[k + 1]!, w: input[k + 2]! });
  return { n: input[0] ?? 0, directed: input[1] === 1, edges };
}

export function encodeGraph(n: number, directed: boolean, edges: readonly (readonly [number, number, number?])[]): number[] {
  return [n, directed ? 1 : 0, ...edges.flatMap(([u, v, w]) => [u, v, w ?? 1])];
}

/** Adjacency lists in `kb_graph`'s order: edges in input order, both ways when undirected.
 * Each entry also carries the edge's index, so a scene can light the edge up. */
export function adjacency(g: Graph): { v: number; w: number; e: number }[][] {
  const adj: { v: number; w: number; e: number }[][] = Array.from({ length: g.n }, () => []);
  g.edges.forEach(({ u, v, w }, e) => {
    adj[u]!.push({ v, w, e });
    if (!g.directed) adj[v]!.push({ v: u, w, e });
  });
  return adj;
}

export function graphCodec(opts: GraphOptions): InputCodec {
  const wMin = opts.weighted ? (opts.wMin ?? 1) : 1;
  const wMax = opts.weighted ? (opts.wMax ?? 9) : 1;
  const minNodes = opts.minNodes ?? 2;
  const maxNodes = opts.maxNodes ?? MAX_NODES;
  const maxEdges = opts.maxEdges ?? MAX_EDGES;
  const choose = opts.directed === "choose";
  const fields: FieldKey[] = choose ? ["count", "directed", "edges"] : ["count", "edges"];
  const check = (input: readonly number[]): InputError | null => {
    if (input.length < 2 || (input.length - 2) % 3 !== 0) return { code: "custom", key: "edge_list" };
    const g = decodeGraph(input);
    if (g.n < minNodes || g.n > maxNodes) return { code: "custom", key: "node_count", params: { min: minNodes, max: maxNodes } };
    if (input[1] !== 0 && input[1] !== 1) return { code: "custom", key: "directed_flag" };
    if (!choose && g.directed !== opts.directed) return { code: "custom", key: "directed_flag" };
    if (g.edges.length < 1 || g.edges.length > maxEdges) return { code: "custom", key: "edge_count", params: { min: 1, max: maxEdges } };
    const seen = new Set<string>();
    for (const { u, v, w } of g.edges) {
      if (u < 0 || v < 0 || u >= g.n || v >= g.n) return { code: "custom", key: "node_range", params: { max: g.n - 1 } };
      if (u === v) return { code: "custom", key: "self_loop", params: { node: u } };
      const key = g.directed ? `${u}>${v}` : `${Math.min(u, v)}-${Math.max(u, v)}`;
      if (seen.has(key)) return { code: "custom", key: "duplicate_edge", params: { u, v } };
      seen.add(key);
      if (w < wMin || w > wMax) return { code: "custom", key: "weight_range", params: { min: wMin, max: wMax } };
    }
    return null;
  };
  return {
    fields,
    format: (input) => {
      const g = decodeGraph(input);
      const edges = g.edges.map(({ u, v, w }) => (opts.weighted ? `${u} ${v} ${w}` : `${u} ${v}`)).join(", ");
      return choose ? [String(g.n), g.directed ? "1" : "0", edges] : [String(g.n), edges];
    },
    parse: (texts) => {
      const nr = tokens(texts[0] ?? "");
      if ("error" in nr) return nr;
      if (nr.values.length !== 1) return fail("one_number", { field: "count" });
      let directed = opts.directed === true;
      if (choose) {
        const d = tokens(texts[1] ?? "");
        if ("error" in d) return d;
        if (d.values.length !== 1 || (d.values[0] !== 0 && d.values[0] !== 1)) return fail("directed_flag");
        directed = d.values[0] === 1;
      }
      const text = (texts[choose ? 2 : 1] ?? "").trim();
      const edges: [number, number, number][] = [];
      for (const chunk of text.split(/[,;\n]+/).map((c) => c.trim()).filter((c) => c !== "")) {
        const r = tokens(chunk);
        if ("error" in r) return r;
        const want = opts.weighted ? 3 : 2;
        if (r.values.length !== want) return fail(opts.weighted ? "edge_weighted" : "edge_pair", { edge: chunk });
        edges.push([r.values[0]!, r.values[1]!, opts.weighted ? r.values[2]! : 1]);
      }
      const values = encodeGraph(nr.values[0]!, directed, edges);
      const e = check(values);
      return e ? { error: e } : { values };
    },
    check,
  };
}

/** A random graph for presets and the random cases: n nodes, about `density` × n edges,
 * connected through a random spanning tree first when `connected`. */
export function randomGraph(
  next: () => number,
  o: { nMin: number; nMax: number; directed: boolean; weighted: boolean; wMin?: number; wMax?: number; density?: number; connected?: boolean; dag?: boolean },
): number[] {
  const n = int(next, o.nMin, o.nMax);
  const target = Math.min(MAX_EDGES, Math.round(n * (o.density ?? 1.4)));
  const w = () => (o.weighted ? int(next, o.wMin ?? 1, o.wMax ?? 9) : 1);
  const edges: [number, number, number][] = [];
  const seen = new Set<string>();
  const add = (u: number, v: number) => {
    if (u === v) return;
    if (o.dag && u > v) [u, v] = [v, u];
    const key = o.directed ? `${u}>${v}` : `${Math.min(u, v)}-${Math.max(u, v)}`;
    if (seen.has(key) || (o.directed && !o.dag && seen.has(`${v}>${u}`))) return;
    seen.add(key);
    edges.push([u, v, w()]);
  };
  if (o.connected) for (let v = 1; v < n; v++) add(int(next, Math.max(0, v - 3), v - 1), v);
  for (let tries = 0; edges.length < target && tries < 200; tries++) add(int(next, 0, n - 1), int(next, 0, n - 1));
  // Shuffle the edge order (it decides the visiting order) without losing determinism.
  for (let k = edges.length - 1; k > 0; k--) {
    const j = int(next, 0, k);
    [edges[k], edges[j]] = [edges[j]!, edges[k]!];
  }
  return encodeGraph(n, o.directed, edges);
}

/** Scene data for a step: node and edge roles, notes under nodes, rings. */
export function graphData(
  g: Graph,
  weighted: boolean,
  s: { nodes?: Record<number, Role>; edges?: Record<number, Role>; notes?: Record<number, string>; marks?: readonly number[]; labels?: Record<number, string> } = {},
): GraphData {
  const nodes: GraphNode[] = Array.from({ length: g.n }, (_, id) => ({ id, role: s.nodes?.[id], note: s.notes?.[id], mark: s.marks?.includes(id) }));
  const edges: GraphEdge[] = g.edges.map((e, id) => ({ id, u: e.u, v: e.v, w: e.w, role: s.edges?.[id], label: s.labels?.[id] }));
  return { n: g.n, directed: g.directed, weighted, nodes, edges };
}
