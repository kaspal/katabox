// B-tree insertion with at most two keys per node (a 2-3 tree). Mirrors
// content/learn/trees/b-tree-insert/sample.{py,ts}: walk down to a leaf, add the key in
// order; a node with three keys splits around its middle key, which moves up into the
// parent (and may split it in turn); a split root grows a new root, so every leaf stays
// at the same depth. Output: the keys level by level, each node followed by a 0. The
// left half of a split keeps the node's identity, so the split animates.
import { Recorder } from "../engine/recorder";
import type { ForestNode, NarrationKey, Role, Step, TracerDef } from "../engine/types";

type Key = NarrationKey<"b-tree-insert">;
const KEYS = [
  "b-tree-insert.ready",
  "b-tree-insert.descend",
  "b-tree-insert.add_key",
  "b-tree-insert.split",
  "b-tree-insert.absorb",
  "b-tree-insert.new_root",
  "b-tree-insert.done",
] as const satisfies readonly Key[];
const say = (key: (typeof KEYS)[number], params?: Record<string, number>) => ({ key, params });

export const MARKERS = ["descend", "add-key", "split", "absorb", "new-root"] as const;
export const MAX_KEYS = 2;

type N = { id: number; keys: number[]; children: number[] };

function trace(values: readonly number[]): Step[] {
  const r = new Recorder([]);
  const nodes = new Map<number, N>();
  let next = 0;
  const make = (keys: number[], children: number[] = []) => {
    const id = next++;
    nodes.set(id, { id, keys, children });
    return id;
  };
  let root = make([]);
  const forest = (roles: Record<number, Role>): ForestNode[] => {
    const out: ForestNode[] = [];
    const walk = (id: number, parent: number | null) => {
      const n = nodes.get(id)!;
      out.push({ id, parent, text: n.keys.length ? n.keys.join(" ") : "·", role: roles[id] });
      n.children.forEach((c) => walk(c, id));
    };
    walk(root, null);
    return out;
  };
  const height = () => {
    let h = 1;
    for (let id = root; nodes.get(id)!.children.length; id = nodes.get(id)!.children[0]!) h++;
    return h;
  };
  const snap = (narration: ReturnType<typeof say>, marker: string | null, roles: Record<number, Role> = {}) =>
    r.snap({ narration, marker, forest: forest(roles), vars: { nodes: nodes.size, height: height() } });

  /** A pending split: the left half keeps `id`, the right half is a new node. */
  const insert = (id: number, key: number): [number, number, number] | null => {
    const node = nodes.get(id)!;
    if (!node.children.length) {
      node.keys.push(key);
      node.keys.sort((a, b) => a - b);
      r.writes++;
      snap(say("b-tree-insert.add_key", { key }), "add-key", { [id]: "key" });
    } else {
      let i = 0;
      while (i < node.keys.length && key >= node.keys[i]!) {
        r.comparisons++;
        i++;
      }
      if (i < node.keys.length) r.comparisons++;
      snap(say("b-tree-insert.descend", { key, child: i + 1 }), "descend", { [id]: "compare", [node.children[i]!]: "pivot" });
      const split = insert(node.children[i]!, key);
      if (split) {
        const [left, middle, right] = split;
        node.keys.splice(i, 0, middle);
        node.children.splice(i, 1, left, right);
        r.writes++;
        snap(say("b-tree-insert.absorb", { middle }), "absorb", { [id]: "key", [left]: "sorted", [right]: "sorted" });
      }
    }
    if (node.keys.length > MAX_KEYS) {
      const mid = Math.floor(node.keys.length / 2);
      const middle = node.keys[mid]!;
      snap(say("b-tree-insert.split", { middle }), "split", { [id]: "swap" });
      const right = make(node.keys.slice(mid + 1), node.children.slice(mid + 1));
      node.keys = node.keys.slice(0, mid);
      node.children = node.children.slice(0, mid + 1);
      r.moves++;
      return [id, middle, right];
    }
    return null;
  };

  snap(say("b-tree-insert.ready", { n: values.length, max: MAX_KEYS }), null);
  for (const v of values) {
    const split = insert(root, v);
    if (split) {
      const [left, middle, right] = split;
      root = make([middle], [left, right]);
      snap(say("b-tree-insert.new_root", { middle }), "new-root", { [root]: "key", [left]: "sorted", [right]: "sorted" });
    }
  }
  const out: number[] = [];
  let level = [root];
  while (level.length) {
    out.push(...level.flatMap((id) => [...nodes.get(id)!.keys, 0]));
    level = level.flatMap((id) => nodes.get(id)!.children);
  }
  snap(say("b-tree-insert.done", { n: values.length, height: height(), splits: r.moves }), null);
  r.steps[r.steps.length - 1]!.result = out;
  return r.steps;
}

export const bTreeInsert: TracerDef = {
  id: "b-tree-insert",
  subject: "trees",
  scene: "tree",
  limits: { min_len: 1, max_len: 12, min: 1, max: 99 },
  input: { kind: "values" },
  markers: MARKERS,
  narrationKeys: KEYS,
  counters: [
    { slot: "comparisons", label: "comparisons" },
    { slot: "writes", label: "inserts" },
    { slot: "moves", label: "splits" },
  ],
  legend: ["compare", "pivot", "key", "swap", "sorted"],
  roleLabels: { compare: "current", pivot: "following", key: "written", swap: "overfull", sorted: "split_half" },
  vars: ["nodes", "height"],
  trace,
};
