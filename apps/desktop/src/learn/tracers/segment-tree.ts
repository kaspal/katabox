// Segment tree for range sums with point updates. Mirrors
// content/learn/trees/segment-tree/sample.{py,ts}: node k covers a range [lo, hi] and
// stores its sum; its children (2k, 2k+1) cover the two halves. A range sum adds the few
// nodes that lie fully inside the query and skips those fully outside; an update rewrites
// one leaf and the sums on its path back to the root. Output: the answers to `sum`.
import { listOpsCodec, type OpSpec } from "../engine/codecs";
import { int } from "../engine/inputs";
import { Recorder } from "../engine/recorder";
import type { ForestNode, Item, NarrationKey, NarrationParams, Role, Step, TracerDef } from "../engine/types";

type Key = NarrationKey<"segment-tree">;
const KEYS = [
  "segment-tree.ready",
  "segment-tree.leaf",
  "segment-tree.merge",
  "segment-tree.query",
  "segment-tree.outside",
  "segment-tree.inside",
  "segment-tree.answer",
  "segment-tree.update",
  "segment-tree.set",
  "segment-tree.recompute",
  "segment-tree.done",
] as const satisfies readonly Key[];
const say = (key: (typeof KEYS)[number], params?: NarrationParams<number>) => ({ key, params });

export const MARKERS = ["leaf", "merge", "outside", "inside", "set", "recompute"] as const;
export const OPS: OpSpec[] = [
  { name: "sum", args: 2 },
  { name: "update", args: 2 },
];
export const BOUNDS = { minLen: 2, maxLen: 8, min: 0, max: 20 };
const codec = listOpsCodec("values", OPS, BOUNDS);
export const enc = (values: number[], ops: [string, number, number][]) => [
  values.length,
  ...values,
  ...ops.flatMap(([name, a, b]) => [OPS.findIndex((o) => o.name === name) + 1, a, b]),
];

function trace(input: readonly number[]): Step[] {
  const { values: a, ops } = codec.split(input);
  const n = a.length;
  const r = new Recorder([]);
  const tree = new Map<number, number>();
  const span = new Map<number, [number, number]>();
  const shape = (node: number, lo: number, hi: number) => {
    span.set(node, [lo, hi]);
    if (lo === hi) return;
    const mid = (lo + hi) >> 1;
    shape(2 * node, lo, mid);
    shape(2 * node + 1, mid + 1, hi);
  };
  shape(1, 0, n - 1);
  const arr = a.map((value, id) => ({ id, value }));
  const out: Item[] = [];
  const snap = (narration: ReturnType<typeof say>, marker: string | null, roles: Record<number, Role> = {}, cells: Record<number, Role> = {}) => {
    const forest: ForestNode[] = [...span.entries()].map(([k, [lo, hi]]) => ({
      id: k,
      parent: k === 1 ? null : k >> 1,
      side: k === 1 ? undefined : k % 2 === 0 ? "L" : "R",
      text: tree.has(k) ? String(tree.get(k)) : "·",
      role: roles[k],
      note: lo === hi ? `${lo}` : `${lo}-${hi}`,
    }));
    r.snap({
      narration,
      marker,
      forest,
      slots: { layout: "row", label: "array", cells: arr.map((c) => ({ ...c })), roles: cells },
      aux: { label: "found", cells: out.map((c) => ({ ...c })), roles: {} },
      vars: {},
    });
  };
  const rangeRoles = (lo: number, hi: number, role: Role) => Object.fromEntries(Array.from({ length: hi - lo + 1 }, (_, k) => [lo + k, role]));

  const build = (node: number, lo: number, hi: number) => {
    if (lo === hi) {
      tree.set(node, a[lo]!);
      r.writes++;
      snap(say("segment-tree.leaf", { i: lo, value: a[lo]! }), "leaf", { [node]: "key" }, { [lo]: "key" });
      return;
    }
    const mid = (lo + hi) >> 1;
    build(2 * node, lo, mid);
    build(2 * node + 1, mid + 1, hi);
    tree.set(node, tree.get(2 * node)! + tree.get(2 * node + 1)!);
    r.writes++;
    snap(say("segment-tree.merge", { lo, hi, sum: tree.get(node)! }), "merge", { [node]: "key", [2 * node]: "compare", [2 * node + 1]: "compare" });
  };
  let acc = 0;
  const query = (node: number, lo: number, hi: number, left: number, right: number): number => {
    r.comparisons++;
    if (right < lo || hi < left) {
      snap(say("segment-tree.outside", { lo, hi }), "outside", { [node]: "out" });
      return 0;
    }
    if (left <= lo && hi <= right) {
      acc += tree.get(node)!;
      snap(say("segment-tree.inside", { lo, hi, sum: tree.get(node)!, acc }), "inside", { [node]: "sorted" }, rangeRoles(lo, hi, "sorted"));
      return tree.get(node)!;
    }
    const mid = (lo + hi) >> 1;
    return query(2 * node, lo, mid, left, right) + query(2 * node + 1, mid + 1, hi, left, right);
  };
  const update = (node: number, lo: number, hi: number, i: number, value: number) => {
    if (lo === hi) {
      tree.set(node, value);
      arr[i]!.value = value;
      r.writes++;
      snap(say("segment-tree.set", { i, value }), "set", { [node]: "key" }, { [i]: "key" });
      return;
    }
    const mid = (lo + hi) >> 1;
    if (i <= mid) update(2 * node, lo, mid, i, value);
    else update(2 * node + 1, mid + 1, hi, i, value);
    tree.set(node, tree.get(2 * node)! + tree.get(2 * node + 1)!);
    r.writes++;
    snap(say("segment-tree.recompute", { lo, hi, sum: tree.get(node)! }), "recompute", { [node]: "swap" });
  };

  snap(say("segment-tree.ready", { n, nodes: span.size }), null);
  build(1, 0, n - 1);
  for (const op of ops) {
    const [x, y] = op.args as [number, number];
    if (op.name === "sum") {
      acc = 0;
      snap(say("segment-tree.query", { left: x, right: y }), null, {}, rangeRoles(x, y, "compare"));
      const total = query(1, 0, n - 1, x, y);
      out.push({ id: out.length, value: total });
      snap(say("segment-tree.answer", { left: x, right: y, sum: total }), null);
    } else {
      snap(say("segment-tree.update", { i: x, value: y }), null, {}, { [x]: "compare" });
      update(1, 0, n - 1, x, y);
    }
  }
  snap(say("segment-tree.done", { answers: out.length, visited: r.comparisons }), null);
  r.steps[r.steps.length - 1]!.result = out.map((c) => c.value);
  return r.steps;
}

export const segmentTree: TracerDef = {
  id: "segment-tree",
  subject: "trees",
  scene: "tree",
  limits: { min_len: 6, max_len: 33, min: 0, max: 20 },
  input: {
    kind: "codec",
    codec: {
      ...codec,
      check: (input) => {
        const e = codec.check!(input);
        if (e) return e;
        const { values, ops } = codec.split(input);
        if (ops.length > 8) return { code: "custom", key: "too_many_ops", params: { max: 8 } };
        for (const op of ops) {
          const [x, y] = op.args as [number, number];
          if (op.name === "sum" && !(0 <= x && x <= y && y < values.length)) return { code: "custom", key: "bad_range", params: { max: values.length - 1 } };
          if (op.name === "update" && !(0 <= x && x < values.length)) return { code: "custom", key: "node_range", params: { max: values.length - 1 } };
        }
        return null;
      },
    },
  },
  presets: {
    random: (next) => {
      const n = int(next, 4, 8);
      const values = Array.from({ length: n }, () => int(next, 0, 20));
      const ops: [string, number, number][] = [];
      const k = int(next, 3, 6);
      for (let j = 0; j < k; j++) {
        if (next() < 0.6) {
          const x = int(next, 0, n - 1);
          ops.push(["sum", x, int(next, x, n - 1)]);
        } else ops.push(["update", int(next, 0, n - 1), int(next, 0, 20)]);
      }
      return enc(values, ops);
    },
    "whole-range": () => enc([5, 3, 8, 6, 1, 4], [["sum", 0, 5], ["update", 2, 0], ["sum", 0, 5]]),
    "small-ranges": () => enc([2, 7, 1, 9, 4, 6, 3, 5], [["sum", 3, 3], ["sum", 2, 5], ["update", 4, 10], ["sum", 1, 6]]),
  },
  markers: MARKERS,
  narrationKeys: KEYS,
  counters: [
    { slot: "comparisons", label: "visits" },
    { slot: "writes", label: "writes" },
  ],
  legend: ["key", "compare", "sorted", "out", "swap"],
  roleLabels: { key: "written", compare: "in_query", sorted: "inside", out: "outside", swap: "recomputed" },
  vars: [],
  trace,
};
