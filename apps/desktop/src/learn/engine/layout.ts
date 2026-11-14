// Tree layout for the Learn scenes (ADR 0029, Phase 2), with d3-hierarchy's tidy tree
// (Reingold-Tilford). A forest is laid out under a virtual root that is never drawn. In a
// binary tree (nodes with a `side`), a missing child gets an invisible placeholder, so a
// lone right child still sits to the right.
import { hierarchy, tree } from "d3-hierarchy";
import type { ForestNode } from "./types";

type LNode = { id: number; kids: LNode[]; phantom?: boolean };

/** Positions in layout units: x in node columns (0 = leftmost), depth in levels. */
export function layoutForest(nodes: readonly ForestNode[]): { at: Map<number, { x: number; depth: number }>; width: number; depth: number } {
  const byId = new Map<number, LNode>();
  for (const n of nodes) byId.set(n.id, { id: n.id, kids: [] });
  const roots: LNode[] = [];
  const binary = nodes.some((n) => n.side !== undefined);
  const sides = new Map<number, { L?: LNode; R?: LNode }>();
  for (const n of nodes) {
    const me = byId.get(n.id)!;
    const parent = n.parent === null ? undefined : byId.get(n.parent);
    if (!parent) {
      roots.push(me);
      continue;
    }
    if (binary && n.side) {
      const s = sides.get(n.parent!) ?? {};
      s[n.side] = me;
      sides.set(n.parent!, s);
    } else parent.kids.push(me);
  }
  let phantom = -1;
  if (binary) {
    for (const [pid, s] of sides) {
      const p = byId.get(pid)!;
      p.kids = [s.L ?? { id: phantom--, kids: [], phantom: true }, s.R ?? { id: phantom--, kids: [], phantom: true }];
    }
  }
  const top: LNode = { id: Number.MIN_SAFE_INTEGER, kids: roots };
  const root = hierarchy<LNode>(top, (d) => d.kids);
  tree<LNode>()
    .nodeSize([1, 1])
    .separation((a, b) => (a.parent === b.parent ? 1 : 1.25))(root);
  const at = new Map<number, { x: number; depth: number }>();
  let min = Infinity;
  let max = -Infinity;
  let deepest = 0;
  for (const d of root.descendants()) {
    if (d.depth === 0 || d.data.phantom) continue;
    min = Math.min(min, d.x);
    max = Math.max(max, d.x);
    deepest = Math.max(deepest, d.depth - 1);
    at.set(d.data.id, { x: d.x, depth: d.depth - 1 });
  }
  if (at.size === 0) return { at, width: 1, depth: 0 };
  for (const [id, p] of at) at.set(id, { x: p.x - min, depth: p.depth });
  return { at, width: max - min + 1, depth: deepest };
}
