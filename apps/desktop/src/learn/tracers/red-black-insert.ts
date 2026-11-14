// Red-black insertion, left-leaning variant (a red link only ever leans left). Mirrors
// content/learn/trees/red-black-insert/sample.{py,ts}: insert as in a BST with a red node,
// then on the way back up fix three patterns: a right-leaning red link (rotate left), two
// reds in a row on the left (rotate right), two red children (flip colours). The root is
// painted black after every insert. Output: pre-order values, then pre-order colours
// (1 red, 0 black). Red nodes are drawn in the "red" role and labelled under the node (a translated note).
import { SORT_INPUT } from "../engine/cases";
import { Recorder } from "../engine/recorder";
import type { ForestNode, NarrationKey, Role, Step, TracerDef } from "../engine/types";

type Key = NarrationKey<"red-black-insert">;
const KEYS = [
  "red-black-insert.ready",
  "red-black-insert.place",
  "red-black-insert.rotate_left",
  "red-black-insert.rotate_right",
  "red-black-insert.flip",
  "red-black-insert.root_black",
  "red-black-insert.done",
] as const satisfies readonly Key[];
const say = (key: (typeof KEYS)[number], params?: Record<string, number>) => ({ key, params });

export const MARKERS = ["place", "rotate-left", "rotate-right", "flip", "root-black"] as const;

type N = { id: number; value: number; left: number | null; right: number | null; red: boolean };

function trace(values: readonly number[]): Step[] {
  const r = new Recorder([]);
  const nodes = new Map<number, N>();
  let root: number | null = null;
  let next = 0;
  const red = (id: number | null) => id !== null && nodes.get(id)!.red;
  const forest = (marks: readonly number[], fresh?: number): ForestNode[] => {
    const out: ForestNode[] = [];
    const walk = (id: number | null, parent: number | null, side?: "L" | "R") => {
      if (id === null) return;
      const n = nodes.get(id)!;
      const role: Role | undefined = id === fresh ? "key" : n.red ? "pivot" : undefined;
      out.push({ id, parent, text: String(n.value), role, side, note: n.red ? "@red" : "@black", mark: marks.includes(id) });
      walk(n.left, id, "L");
      walk(n.right, id, "R");
    };
    walk(root, null);
    return out;
  };
  const snap = (narration: ReturnType<typeof say>, marker: string | null, marks: readonly number[] = [], fresh?: number) =>
    r.snap({ narration, marker, forest: forest(marks, fresh), vars: { root: root === null ? null : nodes.get(root)!.value } });

  const rotLeft = (h: number, attach: (id: number) => void) => {
    const hn = nodes.get(h)!;
    const x = hn.right!;
    const xn = nodes.get(x)!;
    hn.right = xn.left;
    xn.left = h;
    xn.red = hn.red;
    hn.red = true;
    attach(x);
    r.moves++;
    snap(say("red-black-insert.rotate_left", { top: xn.value, down: hn.value }), "rotate-left", [x, h]);
    return x;
  };
  const rotRight = (h: number, attach: (id: number) => void) => {
    const hn = nodes.get(h)!;
    const x = hn.left!;
    const xn = nodes.get(x)!;
    hn.left = xn.right;
    xn.right = h;
    xn.red = hn.red;
    hn.red = true;
    attach(x);
    r.moves++;
    snap(say("red-black-insert.rotate_right", { top: xn.value, down: hn.value }), "rotate-right", [x, h]);
    return x;
  };
  const insert = (h: number | null, value: number, attach: (id: number) => void): number => {
    if (h === null) {
      const nid = next++;
      nodes.set(nid, { id: nid, value, left: null, right: null, red: true });
      attach(nid);
      r.writes++;
      snap(say("red-black-insert.place", { value }), "place", [], nid);
      return nid;
    }
    const hn = nodes.get(h)!;
    r.comparisons++;
    if (value < hn.value) hn.left = insert(hn.left, value, (c) => (hn.left = c));
    else hn.right = insert(hn.right, value, (c) => (hn.right = c));
    let top = h;
    if (red(nodes.get(top)!.right) && !red(nodes.get(top)!.left)) top = rotLeft(top, attach);
    if (red(nodes.get(top)!.left) && red(nodes.get(nodes.get(top)!.left!)!.left)) top = rotRight(top, attach);
    const tn = nodes.get(top)!;
    if (red(tn.left) && red(tn.right)) {
      tn.red = true;
      nodes.get(tn.left!)!.red = false;
      nodes.get(tn.right!)!.red = false;
      r.writes++;
      snap(say("red-black-insert.flip", { value: tn.value }), "flip", [top, tn.left!, tn.right!]);
    }
    return top;
  };

  snap(say("red-black-insert.ready", { n: values.length }), null);
  for (const v of values) {
    root = insert(root, v, (c) => (root = c));
    nodes.get(root)!.red = false;
    snap(say("red-black-insert.root_black", { value: nodes.get(root)!.value }), "root-black", [root]);
  }
  const vals: number[] = [];
  const colors: number[] = [];
  const walk = (id: number | null) => {
    if (id === null) return;
    vals.push(nodes.get(id)!.value);
    colors.push(nodes.get(id)!.red ? 1 : 0);
    walk(nodes.get(id)!.left);
    walk(nodes.get(id)!.right);
  };
  walk(root);
  let blackHeight = 0;
  for (let id = root; id !== null; id = nodes.get(id)!.left) if (!nodes.get(id)!.red) blackHeight++;
  snap(say("red-black-insert.done", { n: values.length, rotations: r.moves, black: blackHeight }), null);
  r.steps[r.steps.length - 1]!.result = [...vals, ...colors];
  return r.steps;
}

export const redBlackInsert: TracerDef = {
  id: "red-black-insert",
  subject: "trees",
  scene: "tree",
  limits: { ...SORT_INPUT, min_len: 1, max_len: 12 },
  input: { kind: "values" },
  markers: MARKERS,
  narrationKeys: KEYS,
  counters: [
    { slot: "comparisons", label: "comparisons" },
    { slot: "moves", label: "rotations" },
    { slot: "writes", label: "writes" },
  ],
  legend: ["key", "pivot", "idle"],
  roleLabels: { key: "new_node", pivot: "red", idle: "black" },
  vars: ["root"],
  trace,
};
