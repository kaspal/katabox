// Segment tree with lazy propagation: add a value to a whole range, sum a range. Mirrors
// content/learn/trees/lazy-propagation/sample.{py,ts}: an add stops at nodes that lie
// fully inside the range, updates their sum and leaves a tag ("add this to my children
// later"); a node passes its tag down only when a later operation has to go below it.
// The array starts at zeros. Output: the answers to `sum`.
import { decodeOps, opsCodec, type OpSpec } from "../engine/codecs";
import { int } from "../engine/inputs";
import { Recorder } from "../engine/recorder";
import type { ForestNode, Item, NarrationKey, Role, Step, TracerDef } from "../engine/types";

type Key = NarrationKey<"lazy-propagation">;
const KEYS = [
  "lazy-propagation.ready",
  "lazy-propagation.add",
  "lazy-propagation.tag",
  "lazy-propagation.push_down",
  "lazy-propagation.recompute",
  "lazy-propagation.query",
  "lazy-propagation.inside",
  "lazy-propagation.answer",
  "lazy-propagation.done",
] as const satisfies readonly Key[];
const say = (key: (typeof KEYS)[number], params?: Record<string, number>) => ({ key, params });

export const MARKERS = ["tag", "push-down", "recompute", "inside"] as const;
export const OPS: OpSpec[] = [
  { name: "add", args: 3 },
  { name: "sum", args: 2 },
];
const codec = opsCodec(OPS, ["count"]);
export const enc = (n: number, ops: [string, number, number, number?][]) => [
  n,
  ...ops.flatMap(([name, a, b, c]) => [OPS.findIndex((o) => o.name === name) + 1, a, b, c ?? 0]),
];

function trace(input: readonly number[]): Step[] {
  const n = input[0]!;
  const ops = decodeOps(input, OPS, codec.width, 1);
  const r = new Recorder([]);
  const tree = new Map<number, number>();
  const lazy = new Map<number, number>();
  const span = new Map<number, [number, number]>();
  const shape = (node: number, lo: number, hi: number) => {
    span.set(node, [lo, hi]);
    tree.set(node, 0);
    lazy.set(node, 0);
    if (lo === hi) return;
    const mid = (lo + hi) >> 1;
    shape(2 * node, lo, mid);
    shape(2 * node + 1, mid + 1, hi);
  };
  shape(1, 0, n - 1);
  const out: Item[] = [];
  const snap = (narration: ReturnType<typeof say>, marker: string | null, roles: Record<number, Role> = {}) => {
    const forest: ForestNode[] = [...span.entries()].map(([k, [lo, hi]]) => ({
      id: k,
      parent: k === 1 ? null : k >> 1,
      side: k === 1 ? undefined : k % 2 === 0 ? "L" : "R",
      text: String(tree.get(k)),
      role: roles[k],
      note: `${lo === hi ? lo : `${lo}-${hi}`}${lazy.get(k) ? ` +${lazy.get(k)}` : ""}`,
      mark: lazy.get(k)! !== 0,
    }));
    r.snap({ narration, marker, forest, aux: { label: "found", cells: out.map((c) => ({ ...c })), roles: {} }, vars: { tags: [...lazy.values()].filter((v) => v !== 0).length } });
  };
  const apply = (node: number, lo: number, hi: number, value: number) => {
    tree.set(node, tree.get(node)! + value * (hi - lo + 1));
    lazy.set(node, lazy.get(node)! + value);
    r.writes++;
  };
  const push = (node: number, lo: number, hi: number) => {
    const tag = lazy.get(node)!;
    if (tag !== 0) {
      const mid = (lo + hi) >> 1;
      apply(2 * node, lo, mid, tag);
      apply(2 * node + 1, mid + 1, hi, tag);
      lazy.set(node, 0);
      r.moves++;
      snap(say("lazy-propagation.push_down", { lo, hi, tag }), "push-down", { [node]: "compare", [2 * node]: "key", [2 * node + 1]: "key" });
    }
  };
  const add = (node: number, lo: number, hi: number, left: number, right: number, value: number) => {
    r.comparisons++;
    if (right < lo || hi < left) return;
    if (left <= lo && hi <= right) {
      apply(node, lo, hi, value);
      snap(say("lazy-propagation.tag", { lo, hi, value, sum: tree.get(node)! }), "tag", { [node]: "key" });
      return;
    }
    push(node, lo, hi);
    const mid = (lo + hi) >> 1;
    add(2 * node, lo, mid, left, right, value);
    add(2 * node + 1, mid + 1, hi, left, right, value);
    tree.set(node, tree.get(2 * node)! + tree.get(2 * node + 1)!);
    snap(say("lazy-propagation.recompute", { lo, hi, sum: tree.get(node)! }), "recompute", { [node]: "swap" });
  };
  let acc = 0;
  const total = (node: number, lo: number, hi: number, left: number, right: number): number => {
    r.comparisons++;
    if (right < lo || hi < left) return 0;
    if (left <= lo && hi <= right) {
      acc += tree.get(node)!;
      snap(say("lazy-propagation.inside", { lo, hi, sum: tree.get(node)!, acc }), "inside", { [node]: "sorted" });
      return tree.get(node)!;
    }
    push(node, lo, hi);
    const mid = (lo + hi) >> 1;
    return total(2 * node, lo, mid, left, right) + total(2 * node + 1, mid + 1, hi, left, right);
  };

  snap(say("lazy-propagation.ready", { n }), null);
  for (const op of ops) {
    const [x, y, v] = op.args as [number, number, number?];
    if (op.name === "add") {
      snap(say("lazy-propagation.add", { left: x, right: y, value: v! }), null);
      add(1, 0, n - 1, x, y, v!);
    } else {
      acc = 0;
      snap(say("lazy-propagation.query", { left: x, right: y }), null);
      const s = total(1, 0, n - 1, x, y);
      out.push({ id: out.length, value: s });
      snap(say("lazy-propagation.answer", { left: x, right: y, sum: s }), null);
    }
  }
  snap(say("lazy-propagation.done", { answers: out.length, pushes: r.moves }), null);
  r.steps[r.steps.length - 1]!.result = out.map((c) => c.value);
  return r.steps;
}

export const lazyPropagation: TracerDef = {
  id: "lazy-propagation",
  subject: "trees",
  scene: "tree",
  limits: { min_len: 5, max_len: 33, min: 0, max: 9 },
  input: {
    kind: "codec",
    codec: {
      ...codec,
      check: (input) => {
        const e = codec.check!(input);
        if (e) return e;
        const n = input[0]!;
        if (n < 2 || n > 8) return { code: "custom", key: "node_count", params: { min: 2, max: 8 } };
        const ops = decodeOps(input, OPS, codec.width, 1);
        if (ops.length > 8) return { code: "custom", key: "too_many_ops", params: { max: 8 } };
        for (const op of ops) {
          const [x, y] = op.args as [number, number];
          if (!(0 <= x && x <= y && y < n)) return { code: "custom", key: "bad_range", params: { max: n - 1 } };
        }
        return null;
      },
    },
  },
  presets: {
    random: (next) => {
      const n = int(next, 4, 8);
      const ops: [string, number, number, number?][] = [];
      const k = int(next, 3, 7);
      for (let j = 0; j < k; j++) {
        const x = int(next, 0, n - 1);
        const y = int(next, x, n - 1);
        if (next() < 0.55) ops.push(["add", x, y, int(next, 1, 9)]);
        else ops.push(["sum", x, y]);
      }
      return enc(n, ops);
    },
    "tag-then-split": () => enc(8, [["add", 0, 7, 2], ["sum", 2, 5], ["add", 4, 5, 3], ["sum", 0, 7]]),
    "overlapping-adds": () => enc(6, [["add", 0, 3, 1], ["add", 2, 5, 4], ["sum", 1, 4], ["sum", 3, 3]]),
  },
  markers: MARKERS,
  narrationKeys: KEYS,
  counters: [
    { slot: "comparisons", label: "visits" },
    { slot: "writes", label: "writes" },
    { slot: "moves", label: "pushes" },
  ],
  legend: ["key", "compare", "swap", "sorted"],
  roleLabels: { key: "tagged", compare: "pushing", swap: "recomputed", sorted: "inside" },
  vars: ["tags"],
  trace,
};
