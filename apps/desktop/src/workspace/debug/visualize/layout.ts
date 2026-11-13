// Positions for the visualizer's SVG views (ADR 0027): no layout library.
import type { GraphModel, TreeModel } from "@/workspace/debug/visualize/model";

export const CELL_H = 32;
export const NODE_R = 17;
export const TREE_DX = 44;
export const TREE_DY = 60;
export const MARGIN = 24;

/** Cell width for a text (monospace, about 7.5 px per character), 36 to 96 px. */
export function cellWidth(text: string): number {
  return Math.max(36, Math.min(96, 16 + text.length * 7.5));
}

/** One width for a row of cells, so indices line up. */
export function rowCellWidth(texts: string[]): number {
  return texts.reduce((w, t) => Math.max(w, cellWidth(t)), 36);
}

export type Point = { x: number; y: number };

/** A tidy binary tree: x by in-order position, y by depth. Never two nodes on one spot. */
export function layoutTree(m: TreeModel): { points: Point[]; width: number; height: number } {
  const order: number[] = [];
  // Iterative in-order walk from the root (node 0).
  const stack: number[] = [];
  let at: number | null = m.nodes.length > 0 ? 0 : null;
  const seen = new Set<number>();
  while (at !== null || stack.length > 0) {
    while (at !== null && !seen.has(at)) {
      stack.push(at);
      at = m.nodes[at].left;
    }
    const n = stack.pop();
    if (n === undefined) break;
    seen.add(n);
    order.push(n);
    at = m.nodes[n].right;
  }
  const points: Point[] = m.nodes.map(() => ({ x: 0, y: 0 }));
  order.forEach((n, i) => {
    points[n] = { x: MARGIN + NODE_R + i * TREE_DX, y: MARGIN + NODE_R + m.nodes[n].depth * TREE_DY };
  });
  const depth = m.nodes.reduce((d, n) => Math.max(d, n.depth), 0);
  return {
    points,
    width: MARGIN * 2 + NODE_R * 2 + Math.max(order.length - 1, 0) * TREE_DX,
    height: MARGIN * 2 + NODE_R * 2 + depth * TREE_DY + 16,
  };
}

/** Nodes on a circle, in order (deterministic). */
export function layoutGraph(m: GraphModel): { points: Point[]; size: number } {
  const n = m.nodes.length;
  const radius = n <= 1 ? 0 : Math.max(70, (n * (NODE_R * 2 + 14)) / (2 * Math.PI));
  const size = 2 * (radius + NODE_R + MARGIN + 16);
  const c = size / 2;
  const points = m.nodes.map((_, i) => {
    const a = -Math.PI / 2 + (2 * Math.PI * i) / Math.max(n, 1);
    return { x: c + radius * Math.cos(a), y: c + radius * Math.sin(a) };
  });
  return { points, size };
}
