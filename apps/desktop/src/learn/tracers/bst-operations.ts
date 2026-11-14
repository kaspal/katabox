// Binary search tree: insert, find, delete. Mirrors content/learn/trees/bst-operations/sample.{py,ts}:
// smaller keys go left, others right. Insert walks down to an empty spot; find walks down
// until it meets the key or falls off; delete splices out a node with at most one child,
// and otherwise copies its successor (the smallest key on its right) up and deletes that
// one instead. Output: the find answers (1/0), 0, then the keys in order.
import { BTree } from "../engine/bst";
import { decodeOps, opsCodec, type OpSpec } from "../engine/codecs";
import { int } from "../engine/inputs";
import { Recorder } from "../engine/recorder";
import type { Item, NarrationKey, Role, Step, TracerDef } from "../engine/types";

type Key = NarrationKey<"bst-operations">;
const KEYS = [
  "bst-operations.ready",
  "bst-operations.descend_left",
  "bst-operations.descend_right",
  "bst-operations.root",
  "bst-operations.attach_left",
  "bst-operations.attach_right",
  "bst-operations.search_left",
  "bst-operations.search_right",
  "bst-operations.found",
  "bst-operations.not_found",
  "bst-operations.missing",
  "bst-operations.visit",
  "bst-operations.splice",
  "bst-operations.successor",
  "bst-operations.copy",
  "bst-operations.done",
] as const satisfies readonly Key[];
const say = (key: (typeof KEYS)[number], params?: Record<string, number>) => ({ key, params });

export const MARKERS = ["descend", "root", "attach-left", "attach-right", "search", "result", "missing", "visit", "splice", "successor", "copy"] as const;
export const OPS: OpSpec[] = [
  { name: "insert", args: 1 },
  { name: "find", args: 1 },
  { name: "delete", args: 1 },
];
const codec = opsCodec(OPS);
const enc = (ops: [string, number][]) => ops.flatMap(([name, v]) => [OPS.findIndex((o) => o.name === name) + 1, v]);

function trace(input: readonly number[]): Step[] {
  const ops = decodeOps(input, OPS, codec.width);
  const r = new Recorder([]);
  const tree = new BTree();
  const out: Item[] = [];
  let nextOut = 0;
  const snap = (narration: ReturnType<typeof say>, marker: string | null, roles: Record<number, Role> = {}, outRole = false) =>
    r.snap({
      narration,
      marker,
      forest: tree.forest(roles),
      aux: { label: "found", cells: out.map((c) => ({ ...c })), roles: outRole ? { [out.length - 1]: "swap" } : {} },
      vars: { size: tree.nodes.size, root: tree.root === null ? null : tree.get(tree.root).value },
    });
  const v = (id: number) => tree.get(id).value;

  /** Replace `id` by `child` under id's parent (or at the root). */
  const splice = (id: number, child: number | null) => {
    const p = tree.get(id).parent;
    if (child !== null) tree.get(child).parent = p;
    if (p === null) tree.root = child;
    else if (tree.get(p).left === id) tree.get(p).left = child;
    else tree.get(p).right = child;
    tree.nodes.delete(id);
  };

  const remove = (id: number | null, value: number): void => {
    if (id === null) {
      snap(say("bst-operations.missing", { value }), "missing");
      return;
    }
    r.comparisons++;
    snap(say("bst-operations.visit", { value, at: v(id) }), "visit", { [id]: "compare" });
    const node = tree.get(id);
    if (value < node.value) return remove(node.left, value);
    if (value > node.value) return remove(node.right, value);
    if (node.left === null || node.right === null) {
      const child = node.left ?? node.right;
      snap(say("bst-operations.splice", { value }), "splice", { [id]: "out", ...(child !== null ? { [child]: "swap" } : {}) });
      splice(id, child);
      r.moves++;
      return;
    }
    let succ = node.right;
    while (tree.get(succ).left !== null) {
      succ = tree.get(succ).left!;
      snap(say("bst-operations.successor", { at: v(succ) }), "successor", { [id]: "key", [succ]: "compare" });
    }
    const was = node.value;
    node.value = v(succ);
    r.writes++;
    snap(say("bst-operations.copy", { from: v(succ), was }), "copy", { [id]: "swap", [succ]: "out" });
    remove(node.right, v(succ));
  };

  snap(say("bst-operations.ready", { n: ops.length }), null);
  for (const op of ops) {
    const value = op.args[0]!;
    if (op.name === "insert") {
      let parent: number | null = null;
      let node = tree.root;
      while (node !== null) {
        parent = node;
        r.comparisons++;
        node = value < v(node) ? tree.get(node).left : tree.get(node).right;
        snap(say(value < v(parent) ? "bst-operations.descend_left" : "bst-operations.descend_right", { value, at: v(parent) }), "descend", { [parent]: "compare" });
      }
      const id = tree.insert(value);
      r.writes++;
      if (parent === null) snap(say("bst-operations.root", { value }), "root", { [id]: "swap" });
      else if (value < v(parent)) snap(say("bst-operations.attach_left", { value, parent: v(parent) }), "attach-left", { [id]: "swap" });
      else snap(say("bst-operations.attach_right", { value, parent: v(parent) }), "attach-right", { [id]: "swap" });
    } else if (op.name === "find") {
      let node = tree.root;
      while (node !== null && v(node) !== value) {
        const from = node;
        r.comparisons++;
        node = value < v(node) ? tree.get(node).left : tree.get(node).right;
        snap(say(value < v(from) ? "bst-operations.search_left" : "bst-operations.search_right", { value, at: v(from) }), "search", { [from]: "compare" });
      }
      out.push({ id: nextOut++, value: node !== null ? 1 : 0 });
      snap(say(node !== null ? "bst-operations.found" : "bst-operations.not_found", { value }), "result", node !== null ? { [node]: "sorted" } : {}, true);
    } else {
      remove(tree.root, value);
    }
  }
  const inorder: number[] = [];
  const walk = (id: number | null) => {
    if (id === null) return;
    walk(tree.get(id).left);
    inorder.push(v(id));
    walk(tree.get(id).right);
  };
  walk(tree.root);
  snap(say("bst-operations.done", { size: inorder.length }), null);
  r.steps[r.steps.length - 1]!.result = [...out.map((c) => c.value), 0, ...inorder];
  return r.steps;
}

export const bstOperations: TracerDef = {
  id: "bst-operations",
  subject: "trees",
  scene: "tree",
  limits: { min_len: 2, max_len: 32, min: 1, max: 99 },
  input: { kind: "codec", codec },
  presets: {
    random: (next) => {
      const ops: [string, number][] = [];
      const keys: number[] = [];
      const n = int(next, 7, 12);
      for (let k = 0; k < n; k++) {
        const roll = next();
        if (k < 4 || roll < 0.55) {
          const key = int(next, 1, 99);
          keys.push(key);
          ops.push(["insert", key]);
        } else if (roll < 0.8) ops.push(["find", next() < 0.6 ? keys[int(next, 0, keys.length - 1)]! : int(next, 1, 99)]);
        else {
          const key = keys[int(next, 0, keys.length - 1)]!;
          ops.push(["delete", key]);
        }
      }
      return enc(ops);
    },
    "delete-with-two-children": () => enc([["insert", 50], ["insert", 30], ["insert", 70], ["insert", 60], ["insert", 80], ["insert", 65], ["delete", 50], ["find", 65], ["find", 50]]),
    skewed: () => enc([["insert", 10], ["insert", 20], ["insert", 30], ["insert", 40], ["insert", 50], ["find", 50], ["find", 5]]),
  },
  markers: MARKERS,
  narrationKeys: KEYS,
  counters: [
    { slot: "comparisons", label: "comparisons" },
    { slot: "writes", label: "writes" },
    { slot: "moves", label: "deletes" },
  ],
  legend: ["compare", "swap", "key", "sorted", "out"],
  roleLabels: { compare: "visiting", swap: "written", key: "being_deleted", sorted: "found", out: "removed" },
  vars: ["size", "root"],
  trace,
};
