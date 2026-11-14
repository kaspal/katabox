// AVL insertion with rotations. Mirrors content/learn/trees/avl-rotations/sample.{py,ts}:
// insert as in a BST, then on the way back up recompute each node's height and balance
// (left height − right height). A balance of ±2 is fixed by one rotation (left-left,
// right-right) or two (left-right, right-left), so the height stays O(log n). Output: the
// final tree in pre-order. Rotations are drawn as moves: nodes keep their identity.
import { SORT_INPUT } from "../engine/cases";
import { Recorder } from "../engine/recorder";
import type { ForestNode, NarrationKey, Role, Step, TracerDef } from "../engine/types";

type Key = NarrationKey<"avl-rotations">;
const KEYS = [
  "avl-rotations.ready",
  "avl-rotations.place",
  "avl-rotations.balanced",
  "avl-rotations.left_heavy",
  "avl-rotations.right_heavy",
  "avl-rotations.rotate_right",
  "avl-rotations.rotate_left",
  "avl-rotations.done",
] as const satisfies readonly Key[];
const say = (key: (typeof KEYS)[number], params?: Record<string, number>) => ({ key, params });

export const MARKERS = ["place", "balance", "rotate-right", "rotate-left"] as const;

type N = { id: number; value: number; left: number | null; right: number | null; height: number };

function trace(values: readonly number[]): Step[] {
  const r = new Recorder([]);
  const nodes = new Map<number, N>();
  let root: number | null = null;
  let next = 0;
  const h = (id: number | null) => (id === null ? 0 : nodes.get(id)!.height);
  const upd = (id: number) => {
    const n = nodes.get(id)!;
    n.height = 1 + Math.max(h(n.left), h(n.right));
  };
  const forest = (roles: Record<number, Role>): ForestNode[] => {
    const out: ForestNode[] = [];
    const walk = (id: number | null, parent: number | null, side?: "L" | "R") => {
      if (id === null) return;
      const n = nodes.get(id)!;
      const b = h(n.left) - h(n.right);
      out.push({ id, parent, text: String(n.value), role: roles[id], side, note: `h${n.height} ${b > 0 ? "+" : ""}${b}` });
      walk(n.left, id, "L");
      walk(n.right, id, "R");
    };
    walk(root, null);
    return out;
  };
  const snap = (narration: ReturnType<typeof say>, marker: string | null, roles: Record<number, Role> = {}) =>
    r.snap({ narration, marker, forest: forest(roles), vars: { root: root === null ? null : nodes.get(root)!.value, height: h(root) } });

  /** `attach` re-links a subtree's new root under its parent (or as the root) at once,
   * so every snapshot shows one whole tree. */
  const rotRight = (y: number, attach: (id: number) => void) => {
    const x = nodes.get(y)!.left!;
    nodes.get(y)!.left = nodes.get(x)!.right;
    nodes.get(x)!.right = y;
    upd(y);
    upd(x);
    attach(x);
    r.moves++;
    snap(say("avl-rotations.rotate_right", { top: nodes.get(x)!.value, down: nodes.get(y)!.value }), "rotate-right", { [x]: "swap", [y]: "swap" });
    return x;
  };
  const rotLeft = (x: number, attach: (id: number) => void) => {
    const y = nodes.get(x)!.right!;
    nodes.get(x)!.right = nodes.get(y)!.left;
    nodes.get(y)!.left = x;
    upd(x);
    upd(y);
    attach(y);
    r.moves++;
    snap(say("avl-rotations.rotate_left", { top: nodes.get(y)!.value, down: nodes.get(x)!.value }), "rotate-left", { [x]: "swap", [y]: "swap" });
    return y;
  };
  const insert = (id: number | null, value: number, attach: (id: number) => void): number => {
    if (id === null) {
      const nid = next++;
      nodes.set(nid, { id: nid, value, left: null, right: null, height: 1 });
      attach(nid);
      r.writes++;
      snap(say("avl-rotations.place", { value }), "place", { [nid]: "key" });
      return nid;
    }
    const node = nodes.get(id)!;
    if (value < node.value) node.left = insert(node.left, value, (c) => (node.left = c));
    else node.right = insert(node.right, value, (c) => (node.right = c));
    upd(id);
    const balance = h(node.left) - h(node.right);
    r.comparisons++;
    const key = balance > 1 ? "avl-rotations.left_heavy" : balance < -1 ? "avl-rotations.right_heavy" : "avl-rotations.balanced";
    snap(say(key, { value: node.value, balance }), "balance", { [id]: Math.abs(balance) > 1 ? "compare" : "sorted" });
    if (balance > 1) {
      if (value >= nodes.get(node.left!)!.value) node.left = rotLeft(node.left!, (c) => (node.left = c));
      return rotRight(id, attach);
    }
    if (balance < -1) {
      if (value < nodes.get(node.right!)!.value) node.right = rotRight(node.right!, (c) => (node.right = c));
      return rotLeft(id, attach);
    }
    return id;
  };

  snap(say("avl-rotations.ready", { n: values.length }), null);
  for (const v of values) root = insert(root, v, (c) => (root = c));
  const pre: number[] = [];
  const walk = (id: number | null) => {
    if (id === null) return;
    pre.push(nodes.get(id)!.value);
    walk(nodes.get(id)!.left);
    walk(nodes.get(id)!.right);
  };
  walk(root);
  snap(say("avl-rotations.done", { n: values.length, height: h(root), rotations: r.moves }), null);
  r.steps[r.steps.length - 1]!.result = pre;
  return r.steps;
}

export const avlRotations: TracerDef = {
  id: "avl-rotations",
  subject: "trees",
  scene: "tree",
  limits: { ...SORT_INPUT, min_len: 1, max_len: 12 },
  input: { kind: "values" },
  markers: MARKERS,
  narrationKeys: KEYS,
  counters: [
    { slot: "writes", label: "inserts" },
    { slot: "moves", label: "rotations" },
  ],
  legend: ["key", "compare", "swap", "sorted"],
  roleLabels: { key: "new_node", compare: "unbalanced", swap: "rotating", sorted: "balanced" },
  vars: ["root", "height"],
  trace,
};
