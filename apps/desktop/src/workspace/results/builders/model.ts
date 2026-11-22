// Visual custom-test builders (expansion M9, ADR 0036): the pure part. Every builder reads
// the parameter's JSON text and writes JSON text back, so the text field stays the source
// of truth and can always be edited by hand.

/** Which builder a parameter type offers (null: none, the JSON field only). */
export type BuilderKind = "tree" | "rows" | "grid" | "graph";

/** The builders offered for `type` (`name` hints a graph), the default first. */
export function buildersFor(type: string, name: string, value: unknown): BuilderKind[] {
  if (type === "TreeNode") return ["tree"];
  if (type === "ListNode") return ["rows"];
  if (type === "string[]") {
    const rows = Array.isArray(value) && value.every((r) => typeof r === "string") ? (value as string[]) : [];
    const grid = rows.length > 0 && rows.every((r) => r.length === rows[0]!.length && r.length > 0);
    return grid ? ["grid", "rows"] : ["rows", "grid"];
  }
  if (type === "int[][]") {
    const pairs =
      Array.isArray(value) &&
      value.length > 0 &&
      value.every((r) => Array.isArray(r) && (r.length === 2 || r.length === 3));
    const named = /edge|link|road|pair|merge|door|friend|connection/i.test(name);
    return pairs && named ? ["graph", "rows"] : ["rows", "graph"];
  }
  if (type.endsWith("[]")) return ["rows"];
  return [];
}

// --- Trees (ADR 0025: level order with nulls, trailing nulls trimmed) -------------------

export type TreeModel = { val: number[]; left: (number | null)[]; right: (number | null)[]; root: number | null };

export function emptyTree(): TreeModel {
  return { val: [], left: [], right: [], root: null };
}

/** Decode a level-order array (with nulls) into nodes; null when malformed. */
export function decodeTree(level: unknown): TreeModel | null {
  if (!Array.isArray(level)) return null;
  const t = emptyTree();
  if (level.length === 0) return t;
  if (typeof level[0] !== "number") return null;
  const add = (v: number) => {
    t.val.push(v);
    t.left.push(null);
    t.right.push(null);
    return t.val.length - 1;
  };
  t.root = add(level[0]);
  const queue = [t.root];
  let i = 1;
  while (i < level.length) {
    const parent = queue.shift();
    if (parent === undefined) return null; // a value with no parent slot
    for (const side of ["left", "right"] as const) {
      if (i >= level.length) break;
      const v = level[i++];
      if (v === null) continue;
      if (typeof v !== "number") return null;
      const id = add(v);
      t[side][parent] = id;
      queue.push(id);
    }
  }
  return t;
}

/** Encode nodes back to the canonical level order. */
export function encodeTree(t: TreeModel): (number | null)[] {
  if (t.root === null) return [];
  const out: (number | null)[] = [];
  const queue: (number | null)[] = [t.root];
  while (queue.length > 0) {
    const n = queue.shift()!;
    if (n === null) {
      out.push(null);
      continue;
    }
    out.push(t.val[n]!);
    queue.push(t.left[n]!, t.right[n]!);
  }
  while (out.length > 0 && out[out.length - 1] === null) out.pop();
  return out;
}

/** Nodes reachable from the root, each with an in-order column and a depth. */
export function placeTree(t: TreeModel): { id: number; x: number; depth: number; parent: number | null }[] {
  const out: { id: number; x: number; depth: number; parent: number | null }[] = [];
  let col = 0;
  const walk = (id: number | null, depth: number, parent: number | null) => {
    if (id === null) return;
    walk(t.left[id]!, depth + 1, id);
    out.push({ id, x: col++, depth, parent });
    walk(t.right[id]!, depth + 1, id);
  };
  walk(t.root, 0, null);
  return out;
}

/** A copy with a new child of `parent` on `side` (or a root when empty). */
export function addChild(t: TreeModel, parent: number | null, side: "left" | "right", value = 0): TreeModel {
  const n: TreeModel = { val: [...t.val, value], left: [...t.left, null], right: [...t.right, null], root: t.root };
  const id = n.val.length - 1;
  if (parent === null) n.root = id;
  else n[side][parent] = id;
  return n;
}

/** A copy without `id` and its subtree. */
export function removeNode(t: TreeModel, id: number): TreeModel {
  const n: TreeModel = { val: [...t.val], left: [...t.left], right: [...t.right], root: t.root };
  if (n.root === id) n.root = null;
  for (let k = 0; k < n.val.length; k++) {
    if (n.left[k] === id) n.left[k] = null;
    if (n.right[k] === id) n.right[k] = null;
  }
  return n;
}

// --- Graphs: int[][] edge lists, [u, v] or [u, v, w] -------------------------------------

/** Node count implied by the edges (max index + 1), at least `min`. */
export function nodeCount(edges: number[][], min = 0): number {
  return Math.max(min, ...edges.flatMap((e) => [e[0]! + 1, e[1]! + 1]), 0);
}

// --- Grids: string[] of equal-length rows ------------------------------------------------

/** The characters a grid cell cycles through: those in the grid, then '.' and '#'. */
export function gridCharset(rows: string[]): string[] {
  const seen = new Set<string>();
  for (const r of rows) for (const c of r) seen.add(c);
  for (const c of [".", "#"]) seen.add(c);
  return [...seen];
}

/** `rows` with cell (r, c) set to `ch`. */
export function setCell(rows: string[], r: number, c: number, ch: string): string[] {
  return rows.map((row, i) => (i === r ? row.slice(0, c) + ch + row.slice(c + 1) : row));
}
