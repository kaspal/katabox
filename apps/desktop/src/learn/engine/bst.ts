// A binary tree for tracers (ADR 0029, Phase 2): nodes by id with left / right / parent,
// built by BST insertion (smaller left, equal or bigger right, as the samples do), and
// snapshotted as forest nodes for the tree scene.
import type { ForestNode, Role } from "./types";

export type BNode = { id: number; value: number; left: number | null; right: number | null; parent: number | null };

export class BTree {
  readonly nodes = new Map<number, BNode>();
  root: number | null = null;
  private next = 0;

  /** Insert by BST order; returns the new node's id. */
  insert(value: number): number {
    const id = this.next++;
    const node: BNode = { id, value, left: null, right: null, parent: null };
    this.nodes.set(id, node);
    if (this.root === null) {
      this.root = id;
      return id;
    }
    let cur = this.nodes.get(this.root)!;
    for (;;) {
      const side = value < cur.value ? "left" : "right";
      const child = cur[side];
      if (child === null) {
        cur[side] = id;
        node.parent = cur.id;
        return id;
      }
      cur = this.nodes.get(child)!;
    }
  }

  get(id: number): BNode {
    return this.nodes.get(id)!;
  }

  /** The tree as forest nodes, with roles and notes by id. */
  forest(roles: Record<number, Role> = {}, notes: Record<number, string> = {}): ForestNode[] {
    const out: ForestNode[] = [];
    const walk = (id: number | null, parent: number | null, side?: "L" | "R") => {
      if (id === null) return;
      const n = this.get(id);
      out.push({ id, parent, text: String(n.value), role: roles[id], side, ...(notes[id] ? { note: notes[id] } : {}) });
      walk(n.left, id, "L");
      walk(n.right, id, "R");
    };
    walk(this.root, null);
    return out;
  }
}

export function bstFrom(values: readonly number[]): BTree {
  const t = new BTree();
  for (const v of values) t.insert(v);
  return t;
}
