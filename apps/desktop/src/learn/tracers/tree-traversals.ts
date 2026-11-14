// Pre-, in- and post-order traversals. Mirrors content/learn/trees/tree-traversals/sample.{py,ts}:
// the values build a binary search tree; each traversal visits every node once, writing it
// before its children (pre), between them (in: sorted order for a BST) or after them
// (post). The visit order collects in the output row; the current traversal's visited
// nodes stay marked.
import { bstFrom, type BTree } from "../engine/bst";
import { SORT_INPUT } from "../engine/cases";
import { Recorder } from "../engine/recorder";
import type { Item, NarrationKey, Role, Step, TracerDef } from "../engine/types";

type Key = NarrationKey<"tree-traversals">;
const KEYS = ["tree-traversals.ready", "tree-traversals.pre", "tree-traversals.in", "tree-traversals.post", "tree-traversals.done"] as const satisfies readonly Key[];
const say = (key: (typeof KEYS)[number], params?: Record<string, number>) => ({ key, params });

export const MARKERS = ["pre", "in", "post"] as const;

function trace(values: readonly number[]): Step[] {
  const tree: BTree = bstFrom(values);
  const r = new Recorder([]);
  const out: Item[] = [];
  let visited = new Set<number>();
  const snap = (narration: ReturnType<typeof say>, marker: string | null, current: number | null) => {
    const roles: Record<number, Role> = {};
    for (const id of visited) roles[id] = "sorted";
    if (current !== null) roles[current] = "key";
    r.snap({
      narration,
      marker,
      forest: tree.forest(roles),
      aux: { label: "order", cells: out.map((c) => ({ ...c })), roles: out.length ? { [out.length - 1]: "swap" } : {} },
      vars: { node: current === null ? null : tree.get(current).value, written: out.length },
    });
  };
  snap(say("tree-traversals.ready", { n: values.length }), null, null);
  const visit = (id: number, kind: "pre" | "in" | "post") => {
    out.push({ id: out.length, value: tree.get(id).value });
    visited.add(id);
    r.comparisons++;
    snap(say(`tree-traversals.${kind}`, { value: tree.get(id).value, k: out.length }), kind, id);
  };
  const pre = (id: number | null) => {
    if (id === null) return;
    visit(id, "pre");
    pre(tree.get(id).left);
    pre(tree.get(id).right);
  };
  const ino = (id: number | null) => {
    if (id === null) return;
    ino(tree.get(id).left);
    visit(id, "in");
    ino(tree.get(id).right);
  };
  const post = (id: number | null) => {
    if (id === null) return;
    post(tree.get(id).left);
    post(tree.get(id).right);
    visit(id, "post");
  };
  pre(tree.root);
  visited = new Set();
  ino(tree.root);
  visited = new Set();
  post(tree.root);
  snap(say("tree-traversals.done", { n: values.length }), null, null);
  r.steps[r.steps.length - 1]!.result = out.map((c) => c.value);
  return r.steps;
}

export const treeTraversals: TracerDef = {
  id: "tree-traversals",
  subject: "trees",
  scene: "tree",
  limits: { ...SORT_INPUT, min_len: 1, max_len: 12 },
  input: { kind: "values" },
  markers: MARKERS,
  narrationKeys: KEYS,
  counters: [{ slot: "comparisons", label: "visits" }],
  legend: ["key", "sorted"],
  roleLabels: { key: "visiting", sorted: "visited" },
  vars: ["node", "written"],
  trace,
};
