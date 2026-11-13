// The visualizer's model (ADR 0027): turns a debugger snapshot into what each view draws.
// Pure functions, no React: detection of each shape, pointers, and the per-problem
// settings.
import type { JsonValue, SnapObject, SnapValue, DebugSnapshot, VarInfo } from "@/bindings";

/** How the user wants a variable drawn ("auto" = by its shape). */
export type ViewAs = "auto" | "graph" | "heap";

/** The per-problem choice (setting `workspace.visualize.<problem>`). */
export type VizSettings = {
  /** Variables to draw; `null` = the default (the problem's parameters). */
  show: string[] | null;
  as: Record<string, ViewAs>;
  /** Overrides of the pointer defaults, both ways. */
  pointers: Record<string, boolean>;
};

export const DEFAULT_SETTINGS: VizSettings = { show: null, as: {}, pointers: {} };

export const settingKey = (problem: string) => `workspace.visualize.${problem}`;

/** Int locals with these names point into arrays by default. */
export const DEFAULT_POINTERS = new Set([
  "i", "j", "k", "l", "r", "c", "lo", "hi", "left", "right", "mid", "start", "end",
  "p", "q", "idx", "pos", "slow", "fast", "top", "row", "col",
]);

/** (row, column) pairs that point into grids. */
export const GRID_POINTERS: [string, string][] = [["r", "c"], ["row", "col"], ["i", "j"]];

/** Drawing limits: the snapshot may hold more; the view says "showing first N". */
export const MAX_TREE_NODES = 127;
export const MAX_LIST_NODES = 60;
export const MAX_GRAPH_NODES = 50;

const VIEW_AS = new Set<ViewAs>(["auto", "graph", "heap"]);

export function parseSettings(v: JsonValue | null): VizSettings | undefined {
  if (typeof v !== "object" || v === null || Array.isArray(v)) return undefined;
  const show = Array.isArray(v.show) ? v.show.filter((s): s is string => typeof s === "string") : null;
  const as: Record<string, ViewAs> = {};
  if (typeof v.as === "object" && v.as !== null && !Array.isArray(v.as)) {
    for (const [k, x] of Object.entries(v.as)) if (VIEW_AS.has(x as ViewAs)) as[k] = x as ViewAs;
  }
  const pointers: Record<string, boolean> = {};
  if (typeof v.pointers === "object" && v.pointers !== null && !Array.isArray(v.pointers)) {
    for (const [k, x] of Object.entries(v.pointers)) if (typeof x === "boolean") pointers[k] = x;
  }
  return { show, as, pointers };
}

/** The variables to ask the debugger for. */
export function chosenNames(settings: VizSettings, params: string[]): string[] {
  return settings.show ?? params;
}

const INT_TYPES = new Set(["int", "number"]);

/** Int locals and their values (pointer candidates). */
export function intLocals(locals: VarInfo[]): Map<string, number> {
  const out = new Map<string, number>();
  for (const v of locals) {
    if (INT_TYPES.has(v.type) && /^-?\d+$/.test(v.value)) out.set(v.name, Number(v.value));
  }
  return out;
}

/** Whether the int local `name` is drawn as a pointer: the user's choice, else the default. */
export function isPointer(name: string, settings: VizSettings): boolean {
  return settings.pointers[name] ?? DEFAULT_POINTERS.has(name);
}

// ---- Values as text ---------------------------------------------------------------------

export function objectOf(snap: DebugSnapshot, v: SnapValue): SnapObject | null {
  return v.t === "ref" ? (snap.objects?.[v.id] ?? null) : null;
}

/** A short text for any value (containers are previewed, nodes by their value). */
export function valueText(snap: DebugSnapshot, v: SnapValue, depth = 0): string {
  if (v.t !== "ref") return v.v;
  const o = objectOf(snap, v);
  if (o === null) return "?";
  if (depth > 1) return "…";
  const join = (xs: SnapValue[], len: number) =>
    xs.slice(0, 4).map((x) => valueText(snap, x, depth + 1)).join(", ") + (len > 4 ? ", …" : "");
  switch (o.kind) {
    case "list":
      return `[${join(o.items, o.len)}]`;
    case "set":
      return `{${join(o.items, o.len)}}`;
    case "map":
      return `{${o.entries.slice(0, 3).map(([k, x]) => `${valueText(snap, k, depth + 1)}: ${valueText(snap, x, depth + 1)}`).join(", ")}${o.len > 3 ? ", …" : ""}}`;
    case "list_node":
      return `ListNode(${valueText(snap, o.val, depth + 1)})`;
    case "tree_node":
      return `TreeNode(${valueText(snap, o.val, depth + 1)})`;
  }
}

// ---- Models -----------------------------------------------------------------------------

export type Cell = { text: string };
export type Pointer = { name: string; index: number };
export type GridPointer = { name: string; r: number; c: number };
export type NodeBox = {
  /** Identity across steps: the input number when the harness built it, else the position. */
  key: string;
  text: string;
  /** Other variables pointing at this node. */
  labels: string[];
};
export type ListNodeBox = NodeBox & { next: string | null };
export type TreeNodeBox = NodeBox & { left: number | null; right: number | null; depth: number };

export type ArrayModel = { kind: "array"; name: string; cells: Cell[]; len: number; pointers: Pointer[] };
export type GridModel = { kind: "grid"; name: string; rows: Cell[][]; len: number; cols: number; pointers: GridPointer[] };
export type LinkedModel = { kind: "linked"; name: string; nodes: ListNodeBox[]; cycleTo: number | null; more: boolean };
export type TreeModel = { kind: "tree"; name: string; nodes: TreeNodeBox[]; more: boolean; heap: boolean };
export type MapModel = { kind: "map"; name: string; type: string; entries: { key: string; value: string }[]; len: number };
export type SetModel = { kind: "set"; name: string; cells: Cell[]; len: number };
export type GraphEdge = { a: number; b: number; count: number; both: boolean };
export type GraphModel = { kind: "graph"; name: string; nodes: string[]; edges: GraphEdge[]; total: number };
export type ScalarModel = { kind: "scalar"; name: string; text: string };
export type Model =
  | ArrayModel
  | GridModel
  | LinkedModel
  | TreeModel
  | MapModel
  | SetModel
  | GraphModel
  | ScalarModel;

/** Which variables point at which object (for node labels). */
function labelsById(snap: DebugSnapshot, except: string): Map<number, string[]> {
  const out = new Map<number, string[]>();
  for (const r of snap.roots ?? []) {
    if (r.name === except || r.value.t !== "ref") continue;
    const list = out.get(r.value.id) ?? [];
    list.push(r.name);
    out.set(r.value.id, list);
  }
  return out;
}

function arrayModel(name: string, snap: DebugSnapshot, items: SnapValue[], len: number, ints: Map<string, number>, settings: VizSettings): ArrayModel {
  const pointers: Pointer[] = [];
  for (const [p, value] of ints) {
    if (p !== name && isPointer(p, settings) && value >= 0 && value <= len) pointers.push({ name: p, index: value });
  }
  return { kind: "array", name, cells: items.map((v) => ({ text: valueText(snap, v) })), len, pointers };
}

function linkedModel(name: string, snap: DebugSnapshot, start: SnapValue): LinkedModel {
  const labels = labelsById(snap, name);
  const nodes: ListNodeBox[] = [];
  const index = new Map<number, number>();
  let at: SnapValue = start;
  let cycleTo: number | null = null;
  while (at.t === "ref") {
    const seen = index.get(at.id);
    if (seen !== undefined) {
      cycleTo = seen;
      break;
    }
    if (nodes.length >= MAX_LIST_NODES) break;
    const o = snap.objects?.[at.id];
    if (!o || o.kind !== "list_node") break;
    index.set(at.id, nodes.length);
    nodes.push({
      key: o.input != null ? `i${o.input}` : `p${nodes.length}`,
      text: valueText(snap, o.val),
      labels: labels.get(at.id) ?? [],
      next: null,
    });
    at = o.next;
  }
  // Cut by the drawing limit, or by the snapshot's caps (an opaque value).
  const more = cycleTo === null && (at.t === "ref" || at.t === "opaque");
  for (let i = 0; i < nodes.length; i++) {
    nodes[i].next = i + 1 < nodes.length ? nodes[i + 1].key : cycleTo !== null && i === nodes.length - 1 ? nodes[cycleTo].key : null;
  }
  return { kind: "linked", name, nodes, cycleTo, more };
}

function treeModel(name: string, snap: DebugSnapshot, root: SnapValue): TreeModel {
  const labels = labelsById(snap, name);
  const nodes: TreeNodeBox[] = [];
  const seen = new Set<number>();
  // Breadth first, keyed by input number or by path (L/R) from the root.
  const queue: { v: SnapValue; parent: number | null; side: "left" | "right" | null; path: string; depth: number }[] = [
    { v: root, parent: null, side: null, path: "", depth: 0 },
  ];
  let more = false;
  for (let k = 0; k < queue.length; k++) {
    const { v, parent, side, path, depth } = queue[k];
    if (v.t !== "ref" || seen.has(v.id)) continue;
    const o = snap.objects?.[v.id];
    if (!o || o.kind !== "tree_node") continue;
    if (nodes.length >= MAX_TREE_NODES) {
      more = true;
      break;
    }
    seen.add(v.id);
    const at = nodes.length;
    nodes.push({
      key: o.input != null ? `i${o.input}` : `p${path}`,
      text: valueText(snap, o.val),
      labels: labels.get(v.id) ?? [],
      left: null,
      right: null,
      depth,
    });
    if (parent !== null && side !== null) nodes[parent][side] = at;
    queue.push({ v: o.left, parent: at, side: "left", path: path + "L", depth: depth + 1 });
    queue.push({ v: o.right, parent: at, side: "right", path: path + "R", depth: depth + 1 });
  }
  return { kind: "tree", name, nodes, more, heap: false };
}

/** An array as a binary heap (children of i at 2i+1 and 2i+2). */
function heapModel(name: string, snap: DebugSnapshot, items: SnapValue[]): TreeModel {
  const n = Math.min(items.length, MAX_TREE_NODES);
  const nodes: TreeNodeBox[] = [];
  for (let i = 0; i < n; i++) {
    nodes.push({
      key: `p${i}`,
      text: valueText(snap, items[i]),
      labels: [],
      left: 2 * i + 1 < n ? 2 * i + 1 : null,
      right: 2 * i + 2 < n ? 2 * i + 2 : null,
      depth: Math.floor(Math.log2(i + 1)),
    });
  }
  return { kind: "tree", name, nodes, more: items.length > n, heap: true };
}

function intOf(v: SnapValue): number | null {
  return v.t === "int" && /^-?\d+$/.test(v.v) ? Number(v.v) : null;
}

/** A graph from an adjacency list (`int[][]`), a 0/1 matrix, or a dict of lists. */
function graphModel(name: string, snap: DebugSnapshot, o: SnapObject): GraphModel | null {
  const labels: string[] = [];
  const indexOf = new Map<string, number>();
  const node = (label: string) => {
    let i = indexOf.get(label);
    if (i === undefined) {
      i = labels.length;
      indexOf.set(label, i);
      labels.push(label);
    }
    return i;
  };
  const raw: [number, number][] = [];
  if (o.kind === "list") {
    const rows = o.items.map((r) => objectOf(snap, r));
    if (rows.some((r) => r === null || r.kind !== "list")) return null;
    const lists = rows as Extract<SnapObject, { kind: "list" }>[];
    const n = lists.length;
    for (let i = 0; i < n; i++) node(String(i));
    const matrix = n > 0 && lists.every((r) => r.len === n && r.items.every((x) => intOf(x) !== null && Math.abs(intOf(x)!) <= 1));
    lists.forEach((row, i) => {
      row.items.forEach((x, j) => {
        if (matrix) {
          if (intOf(x) !== 0) raw.push([i, j]);
          return;
        }
        // An adjacency list: a neighbour, or [neighbour, weight].
        const first = x.t === "ref" ? (objectOf(snap, x) as Extract<SnapObject, { kind: "list" }> | null)?.items?.[0] : x;
        const to = first ? intOf(first) : null;
        if (to !== null) raw.push([i, node(String(to))]);
      });
    });
  } else if (o.kind === "map") {
    for (const [k, x] of o.entries) {
      const from = node(valueText(snap, k));
      const list = objectOf(snap, x);
      if (!list || (list.kind !== "list" && list.kind !== "set")) return null;
      for (const y of list.items) raw.push([from, node(valueText(snap, y))]);
    }
  } else {
    return null;
  }
  const total = labels.length;
  const keep = Math.min(total, MAX_GRAPH_NODES);
  // One edge per pair of nodes: a count for duplicates, both directions merged.
  const byPair = new Map<string, GraphEdge>();
  for (const [x, y] of raw) {
    if (x >= keep || y >= keep) continue;
    const [a, b] = x <= y ? [x, y] : [y, x];
    const key = `${a}-${b}`;
    const e = byPair.get(key);
    if (!e) byPair.set(key, { a: x, b: y, count: 1, both: false });
    else if (e.a === x && e.b === y) e.count++;
    else e.both = true;
  }
  return { kind: "graph", name, nodes: labels.slice(0, keep), edges: [...byPair.values()], total };
}

/** The model for one root, drawn as the user asked (or by its shape). */
export function modelFor(
  name: string,
  value: SnapValue,
  snap: DebugSnapshot,
  locals: VarInfo[],
  settings: VizSettings,
): Model {
  const as = settings.as[name] ?? "auto";
  const o = objectOf(snap, value);
  if (o === null) return { kind: "scalar", name, text: valueText(snap, value) };
  if (as === "graph") {
    const g = graphModel(name, snap, o);
    if (g) return g;
  }
  const ints = intLocals(locals);
  switch (o.kind) {
    case "list_node":
      return linkedModel(name, snap, value);
    case "tree_node":
      return treeModel(name, snap, value);
    case "map":
      return {
        kind: "map",
        name,
        type: o.type,
        entries: o.entries.map(([k, x]) => ({ key: valueText(snap, k), value: valueText(snap, x) })),
        len: o.len,
      };
    case "set":
      return { kind: "set", name, cells: o.items.map((x) => ({ text: valueText(snap, x) })), len: o.len };
    case "list": {
      if (as === "heap") return heapModel(name, snap, o.items);
      const rows = o.items.map((r) => objectOf(snap, r));
      const grid =
        o.items.length > 0 &&
        rows.every((r) => r !== null && r.kind === "list" && r.items.every((x) => x.t !== "ref"));
      if (grid) {
        const lists = rows as Extract<SnapObject, { kind: "list" }>[];
        const cols = Math.max(...lists.map((r) => r.len));
        const pointers: GridPointer[] = [];
        for (const [rn, cn] of GRID_POINTERS) {
          const r = ints.get(rn);
          const c = ints.get(cn);
          if (r !== undefined && c !== undefined && isPointer(rn, settings) && isPointer(cn, settings) && r >= 0 && r < o.len && c >= 0 && c < cols) {
            pointers.push({ name: `${rn},${cn}`, r, c });
          }
        }
        return {
          kind: "grid",
          name,
          rows: lists.map((r) => r.items.map((x) => ({ text: valueText(snap, x) }))),
          len: o.len,
          cols,
          pointers,
        };
      }
      return arrayModel(name, snap, o.items, o.len, ints, settings);
    }
  }
}

/** The models to draw: every root the user chose (plus node locals the harness added are
 * only labels, not cards, unless chosen). */
export function modelsOf(snap: DebugSnapshot, chosen: string[], locals: VarInfo[], settings: VizSettings): Model[] {
  const out: Model[] = [];
  for (const name of chosen) {
    const root = (snap.roots ?? []).find((r) => r.name === name);
    if (root) out.push(modelFor(name, root.value, snap, locals, settings));
  }
  return out;
}

/** What a view shows fewer of than there is ("showing first N of M"), or null. */
export function shownOf(m: Model): { shown: number; total: number | null } | null {
  switch (m.kind) {
    case "array":
      return m.cells.length < m.len ? { shown: m.cells.length, total: m.len } : null;
    case "grid":
      return m.rows.length < m.len ? { shown: m.rows.length, total: m.len } : null;
    case "map":
      return m.entries.length < m.len ? { shown: m.entries.length, total: m.len } : null;
    case "set":
      return m.cells.length < m.len ? { shown: m.cells.length, total: m.len } : null;
    case "graph":
      return m.nodes.length < m.total ? { shown: m.nodes.length, total: m.total } : null;
    case "linked":
    case "tree":
      return m.more ? { shown: m.nodes.length, total: null } : null;
    default:
      return null;
  }
}
