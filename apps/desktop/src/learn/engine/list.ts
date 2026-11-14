// Linked-list helpers for tracers (ADR 0029, Phase 2).
import type { ListData, ListNode, PointerTone, Role } from "./types";

export type LNode = { id: number; value: number; next: number | null; prev: number | null; text?: string };

/** The ids reached from `head` by `next`, stopping at null or a repeat (a cycle). */
export function chain(nodes: ReadonlyMap<number, LNode>, head: number | null): number[] {
  const out: number[] = [];
  const seen = new Set<number>();
  for (let id = head; id !== null && !seen.has(id); id = nodes.get(id)!.next) {
    seen.add(id);
    out.push(id);
  }
  return out;
}

/** A snapshot of the nodes in `order`, with roles and labels by node id. */
export function listData(
  nodes: ReadonlyMap<number, LNode>,
  order: readonly number[],
  opts: {
    roles?: Record<number, Role>;
    labels?: [string, number | null, PointerTone][];
    doubly?: boolean;
  } = {},
): ListData {
  const shown = order.filter((id) => nodes.has(id));
  const next: Record<number, number | null> = {};
  const prev: Record<number, number | null> = {};
  const list: ListNode[] = shown.map((id) => {
    const n = nodes.get(id)!;
    next[id] = n.next;
    prev[id] = n.prev;
    return { id, value: n.value, role: opts.roles?.[id], ...(n.text ? { text: n.text } : {}) };
  });
  return {
    nodes: list,
    next,
    ...(opts.doubly ? { prev } : {}),
    labels: (opts.labels ?? []).filter((l): l is [string, number, PointerTone] => l[1] !== null).map(([label, id, tone]) => ({ label, id, tone })),
  };
}
