// Level-order traversal (breadth first). Mirrors content/learn/trees/level-order/sample.{py,ts}:
// a queue starts with the root; take the front node, write it, and queue its children left
// then right. Nodes come out level by level. The queue is shown under the tree.
import { bstFrom } from "../engine/bst";
import { SORT_INPUT } from "../engine/cases";
import { Recorder } from "../engine/recorder";
import type { Item, NarrationKey, Role, Step, TracerDef } from "../engine/types";

type Key = NarrationKey<"level-order">;
const KEYS = ["level-order.ready", "level-order.visit", "level-order.enqueue", "level-order.done"] as const satisfies readonly Key[];
const say = (key: (typeof KEYS)[number], params?: Record<string, number>) => ({ key, params });

export const MARKERS = ["visit", "enqueue"] as const;

function trace(values: readonly number[]): Step[] {
  const tree = bstFrom(values);
  const r = new Recorder([]);
  const out: Item[] = [];
  const queue: number[] = [tree.root!];
  const visited = new Set<number>();
  const snap = (narration: ReturnType<typeof say>, marker: string | null, current: number | null, added: number | null = null) => {
    const roles: Record<number, Role> = {};
    for (const id of visited) roles[id] = "sorted";
    for (const id of queue) roles[id] = "pivot";
    if (current !== null) roles[current] = "key";
    if (added !== null) roles[added] = "swap";
    r.snap({
      narration,
      marker,
      forest: tree.forest(roles),
      slots: { layout: "row", label: "queue", cells: queue.map((id) => ({ id, value: tree.get(id).value })) },
      aux: { label: "order", cells: out.map((c) => ({ ...c })) },
      vars: { node: current === null ? null : tree.get(current).value, queued: queue.length },
    });
  };
  snap(say("level-order.ready", { n: values.length, root: tree.get(tree.root!).value }), null, null);
  while (queue.length > 0) {
    const id = queue.shift()!;
    visited.add(id);
    out.push({ id: out.length, value: tree.get(id).value });
    r.comparisons++;
    snap(say("level-order.visit", { value: tree.get(id).value }), "visit", id);
    for (const child of [tree.get(id).left, tree.get(id).right]) {
      if (child !== null) {
        queue.push(child);
        r.writes++;
        snap(say("level-order.enqueue", { value: tree.get(child).value, parent: tree.get(id).value }), "enqueue", id, child);
      }
    }
  }
  snap(say("level-order.done", { n: values.length }), null, null);
  r.steps[r.steps.length - 1]!.result = out.map((c) => c.value);
  return r.steps;
}

export const levelOrder: TracerDef = {
  id: "level-order",
  subject: "trees",
  scene: "tree",
  limits: { ...SORT_INPUT, min_len: 1, max_len: 12 },
  input: { kind: "values" },
  markers: MARKERS,
  narrationKeys: KEYS,
  counters: [
    { slot: "comparisons", label: "visits" },
    { slot: "writes", label: "enqueued" },
  ],
  legend: ["key", "pivot", "swap", "sorted"],
  roleLabels: { key: "visiting", pivot: "in_queue", swap: "entering", sorted: "visited" },
  vars: ["node", "queued"],
  trace,
};
