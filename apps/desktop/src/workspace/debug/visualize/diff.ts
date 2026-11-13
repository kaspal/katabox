// What changed since the previous step (ADR 0027): a changed cell, a moved pointer, a
// relinked node, a map key added or changed. Nodes are matched by their key: the input
// number when the harness built them, else their position (nodes the solution creates).
import type { LinkedModel, Model, TreeModel } from "@/workspace/debug/visualize/model";

export type Changes = {
  /** Array/set cells by index, grid cells as "r,c", map entries by key. */
  cells: Set<string>;
  /** Node keys whose value or link changed, or that are new. */
  nodes: Set<string>;
  /** Pointer or label names that moved or appeared. */
  pointers: Set<string>;
  /** Graph edges ("a-b") that are new. */
  edges: Set<string>;
  /** Map keys that are gone. */
  removed: string[];
};

export const NO_CHANGES: Changes = {
  cells: new Set(),
  nodes: new Set(),
  pointers: new Set(),
  edges: new Set(),
  removed: [],
};

export function hasChanges(c: Changes): boolean {
  return c.cells.size + c.nodes.size + c.pointers.size + c.edges.size + c.removed.length > 0;
}

/** Every node of some models: its value and links by key, and which node each label is on. */
export type NodeIndex = { nodes: Map<string, string>; labels: Map<string, string> };

function stateOf(m: LinkedModel | TreeModel, i: number): string {
  const n = m.nodes[i];
  if (m.kind === "linked") return `${n.text}|next=${(n as LinkedModel["nodes"][number]).next}`;
  const t = n as TreeModel["nodes"][number];
  const key = (j: number | null) => (j === null ? null : m.nodes[j].key);
  return `${t.text}|l=${key(t.left)}|r=${key(t.right)}`;
}

/** The nodes of the previous step's drawings. Only input-numbered nodes are comparable
 * across drawings; `positional` adds the others (for the same drawing). */
export function nodeIndex(models: Model[], positional = false): NodeIndex {
  const out: NodeIndex = { nodes: new Map(), labels: new Map() };
  for (const m of models) {
    if (m.kind !== "linked" && m.kind !== "tree") continue;
    m.nodes.forEach((n, i) => {
      if (positional || n.key.startsWith("i")) out.nodes.set(n.key, stateOf(m, i));
      for (const l of n.labels) out.labels.set(l, n.key);
    });
  }
  return out;
}

/** Changes from `prev` (the same variable at the previous step) to `cur`. `known` holds
 * the previous step's nodes from every drawing, so a node that moved from one list to
 * another (a reversal's `back` to `prev`) is still compared by its input number. */
export function diffModels(prev: Model | undefined, cur: Model, known?: NodeIndex): Changes {
  if (cur.kind === "linked" || cur.kind === "tree") return diffNodes(prev, cur, known);
  if (!prev || prev.kind !== cur.kind) return NO_CHANGES;
  const out: Changes = { cells: new Set(), nodes: new Set(), pointers: new Set(), edges: new Set(), removed: [] };
  switch (cur.kind) {
    case "array":
    case "set": {
      const p = prev as typeof cur;
      cur.cells.forEach((c, i) => {
        if (p.cells[i]?.text !== c.text) out.cells.add(String(i));
      });
      if (cur.kind === "array" && p.kind === "array") {
        for (const ptr of cur.pointers) {
          const before = p.pointers.find((x) => x.name === ptr.name);
          if (!before || before.index !== ptr.index) out.pointers.add(ptr.name);
        }
      }
      break;
    }
    case "grid": {
      const p = prev as typeof cur;
      cur.rows.forEach((row, r) =>
        row.forEach((c, k) => {
          if (p.rows[r]?.[k]?.text !== c.text) out.cells.add(`${r},${k}`);
        }),
      );
      for (const ptr of cur.pointers) {
        const before = p.pointers.find((x) => x.name === ptr.name);
        if (!before || before.r !== ptr.r || before.c !== ptr.c) out.pointers.add(ptr.name);
      }
      break;
    }
    case "map": {
      const p = prev as typeof cur;
      const before = new Map(p.entries.map((e) => [e.key, e.value]));
      const now = new Set(cur.entries.map((e) => e.key));
      for (const e of cur.entries) if (before.get(e.key) !== e.value) out.cells.add(e.key);
      // Only keys the previous step listed in full can be "removed" (not cut by caps).
      if (p.entries.length === p.len) for (const k of before.keys()) if (!now.has(k)) out.removed.push(k);
      break;
    }
    case "graph": {
      const p = prev as typeof cur;
      const name = (m: typeof cur, i: number) => m.nodes[i];
      const before = new Set(p.edges.map((e) => `${name(p, e.a)}-${name(p, e.b)}`));
      for (const e of cur.edges) {
        const k = `${name(cur, e.a)}-${name(cur, e.b)}`;
        if (!before.has(k)) out.edges.add(k);
      }
      break;
    }
    case "scalar": {
      const p = prev as typeof cur;
      if (p.text !== cur.text) out.cells.add("0");
      break;
    }
  }
  return out;
}

function diffNodes(prev: Model | undefined, cur: LinkedModel | TreeModel, known?: NodeIndex): Changes {
  const local = prev && prev.kind === cur.kind ? nodeIndex([prev], true) : null;
  if (!local && !known) return NO_CHANGES;
  const out: Changes = { cells: new Set(), nodes: new Set(), pointers: new Set(), edges: new Set(), removed: [] };
  cur.nodes.forEach((n, i) => {
    const before = (n.key.startsWith("i") ? known?.nodes.get(n.key) : undefined) ?? local?.nodes.get(n.key);
    // New here, or its value or a link changed.
    if (before !== stateOf(cur, i)) out.nodes.add(n.key);
    for (const l of n.labels) if ((known?.labels.get(l) ?? local?.labels.get(l)) !== n.key) out.pointers.add(l);
  });
  return out;
}
