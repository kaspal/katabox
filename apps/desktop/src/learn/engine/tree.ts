// Tree layout for recursion trees (ADR 0029): positions come from the final tree, so a
// node never moves as the tree grows. Leaves take consecutive columns in call order and a
// parent sits midway over its first and last child (a range-split recursion's leaves land
// over their array positions).
import type { Step, TreeNode } from "./types";

export type Placed = { x: number; depth: number };

export function layoutTree(nodes: readonly TreeNode[]): { at: Map<number, Placed>; columns: number; depth: number } {
  const children = new Map<number | null, number[]>();
  for (const n of [...nodes].sort((a, b) => a.id - b.id)) {
    const list = children.get(n.parent) ?? [];
    list.push(n.id);
    children.set(n.parent, list);
  }
  const at = new Map<number, Placed>();
  let next = 0;
  let deepest = 0;
  const place = (id: number, depth: number): number => {
    deepest = Math.max(deepest, depth);
    const kids = children.get(id) ?? [];
    let x: number;
    if (kids.length === 0) x = next++;
    else {
      const xs = kids.map((k) => place(k, depth + 1));
      x = (xs[0]! + xs[xs.length - 1]!) / 2;
    }
    at.set(id, { x, depth });
    return x;
  };
  for (const root of children.get(null) ?? []) place(root, 0);
  return { at, columns: Math.max(1, next), depth: deepest };
}

/** Every node that appears in any step (the final shape of the tree). */
export function allNodes(steps: readonly Step[]): TreeNode[] {
  const seen = new Map<number, TreeNode>();
  for (const s of steps) for (const n of s.tree ?? []) seen.set(n.id, n);
  return [...seen.values()];
}

/** Stamp every snapshot's tree nodes with their final position (`x`, `depth`) and the
 * tree's size, so scenes lay out from the step alone. Call once at the end of a trace. */
export function placeTrees(steps: Step[]): Step[] {
  const { at, columns, depth } = layoutTree(allNodes(steps));
  for (const s of steps) {
    if (!s.tree) continue;
    s.tree = s.tree.map((n) => ({ ...n, ...at.get(n.id)! }));
    s.treeSize = { columns, depth };
  }
  return steps;
}
