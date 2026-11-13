// What changed since the previous step (ADR 0027): a changed cell, a moved pointer, a
// relinked node, a map key added or changed. Nodes are matched by their key: the input
// number when the harness built them, else their position (nodes the solution creates).
import type { Model } from "@/workspace/debug/visualize/model";

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

/** Changes from `prev` (the same variable at the previous step) to `cur`. */
export function diffModels(prev: Model | undefined, cur: Model): Changes {
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
    case "linked": {
      const p = prev as typeof cur;
      const before = new Map(p.nodes.map((n) => [n.key, n]));
      const labelAt = new Map<string, string>();
      for (const n of p.nodes) for (const l of n.labels) labelAt.set(l, n.key);
      for (const n of cur.nodes) {
        const b = before.get(n.key);
        if (!b || b.text !== n.text || b.next !== n.next) out.nodes.add(n.key);
        for (const l of n.labels) if (labelAt.get(l) !== n.key) out.pointers.add(l);
      }
      break;
    }
    case "tree": {
      const p = prev as typeof cur;
      const keyOf = (m: typeof cur, i: number | null) => (i === null ? null : m.nodes[i].key);
      const before = new Map(p.nodes.map((n) => [n.key, n]));
      const labelAt = new Map<string, string>();
      for (const n of p.nodes) for (const l of n.labels) labelAt.set(l, n.key);
      for (const n of cur.nodes) {
        const b = before.get(n.key);
        if (
          !b ||
          b.text !== n.text ||
          keyOf(p, b.left) !== keyOf(cur, n.left) ||
          keyOf(p, b.right) !== keyOf(cur, n.right)
        ) {
          out.nodes.add(n.key);
        }
        for (const l of n.labels) if (labelAt.get(l) !== n.key) out.pointers.add(l);
      }
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
