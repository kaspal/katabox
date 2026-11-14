// Fenwick tree (binary indexed tree) for prefix sums with point adds. Mirrors
// content/learn/trees/fenwick-tree/sample.{py,ts}: slot i (1-based) stores the sum of the
// last lowbit(i) values up to i. A prefix sum reads slots while stripping the lowest set
// bit (i -= i & -i); an add touches slots while adding it (i += i & -i); both take
// O(log n) steps. Drawn as the read tree: slot i hangs under i − lowbit(i), so a prefix
// sum climbs to the root. The array starts at zeros. Output: the answers to `sum`.
import { decodeOps, opsCodec, type OpSpec } from "../engine/codecs";
import { int } from "../engine/inputs";
import { Recorder } from "../engine/recorder";
import type { ForestNode, Item, NarrationKey, NarrationParams, Role, Step, TracerDef } from "../engine/types";

type Key = NarrationKey<"fenwick-tree">;
const KEYS = [
  "fenwick-tree.ready",
  "fenwick-tree.start_add",
  "fenwick-tree.add",
  "fenwick-tree.start_sum",
  "fenwick-tree.read",
  "fenwick-tree.answer",
  "fenwick-tree.done",
] as const satisfies readonly Key[];
const say = (key: (typeof KEYS)[number], params?: NarrationParams<number>) => ({ key, params });

export const MARKERS = ["add", "read"] as const;
export const OPS: OpSpec[] = [
  { name: "add", args: 2 },
  { name: "sum", args: 1 },
];
const codec = opsCodec(OPS, ["count"]);
export const enc = (n: number, ops: [string, number, number?][]) => [
  n,
  ...ops.flatMap(([name, a, b]) => [OPS.findIndex((o) => o.name === name) + 1, a, b ?? 0]),
];
const low = (i: number) => i & -i;

function trace(input: readonly number[]): Step[] {
  const n = input[0]!;
  const ops = decodeOps(input, OPS, codec.width, 1);
  const r = new Recorder([]);
  const tree = new Array<number>(n + 1).fill(0);
  const arr = Array.from({ length: n }, (_, id) => ({ id, value: 0 }));
  const out: Item[] = [];
  let acc = 0;
  const snap = (narration: ReturnType<typeof say>, marker: string | null, roles: Record<number, Role> = {}, cells: Record<number, Role> = {}) => {
    const forest: ForestNode[] = [{ id: 0, parent: null, text: "·" }];
    for (let i = 1; i <= n; i++) {
      const from = i - low(i);
      forest.push({ id: i, parent: from, text: String(tree[i]), role: roles[i], note: from === i - 1 ? `[${i}]` : `[${from + 1}-${i}]` });
    }
    r.snap({
      narration,
      marker,
      forest,
      slots: { layout: "row", label: "array", cells: arr.map((c) => ({ ...c })), roles: cells },
      aux: { label: "found", cells: out.map((c) => ({ ...c })), roles: {} },
      vars: { acc },
    });
  };

  snap(say("fenwick-tree.ready", { n }), null);
  for (const op of ops) {
    if (op.name === "add") {
      const [p, v] = op.args as [number, number];
      arr[p]!.value += v;
      snap(say("fenwick-tree.start_add", { i: p, value: v }), null, {}, { [p]: "key" });
      for (let i = p + 1; i <= n; i += low(i)) {
        tree[i] += v;
        r.writes++;
        snap(say("fenwick-tree.add", { slot: i, value: v, total: tree[i]!, low: low(i) }), "add", { [i]: "key" }, { [p]: "key" });
      }
    } else {
      const [p] = op.args as [number];
      acc = 0;
      const covered = Object.fromEntries(Array.from({ length: p + 1 }, (_, k) => [k, "compare" as Role]));
      snap(say("fenwick-tree.start_sum", { i: p }), null, {}, covered);
      for (let i = p + 1; i > 0; i -= low(i)) {
        acc += tree[i]!;
        r.comparisons++;
        const part = Object.fromEntries(Array.from({ length: low(i) }, (_, k) => [i - low(i) + k, "sorted" as Role]));
        snap(say("fenwick-tree.read", { slot: i, value: tree[i]!, acc }), "read", { [i]: "sorted" }, { ...covered, ...part });
      }
      out.push({ id: out.length, value: acc });
      snap(say("fenwick-tree.answer", { i: p, sum: acc }), null);
    }
  }
  snap(say("fenwick-tree.done", { answers: out.length, reads: r.comparisons, writes: r.writes }), null);
  r.steps[r.steps.length - 1]!.result = out.map((c) => c.value);
  return r.steps;
}

export const fenwickTree: TracerDef = {
  id: "fenwick-tree",
  subject: "trees",
  scene: "tree",
  limits: { min_len: 4, max_len: 31, min: 0, max: 12 },
  input: {
    kind: "codec",
    codec: {
      ...codec,
      check: (input) => {
        const e = codec.check!(input);
        if (e) return e;
        const n = input[0]!;
        if (n < 2 || n > 12) return { code: "custom", key: "node_count", params: { min: 2, max: 12 } };
        const ops = decodeOps(input, OPS, codec.width, 1);
        if (ops.length > 10) return { code: "custom", key: "too_many_ops", params: { max: 10 } };
        for (const op of ops) {
          if (op.args[0]! >= n) return { code: "custom", key: "node_range", params: { max: n - 1 } };
          if (op.name === "add" && (op.args[1]! < 1 || op.args[1]! > 9)) return { code: "custom", key: "value_range", params: { min: 1, max: 9 } };
        }
        return null;
      },
    },
  },
  presets: {
    random: (next) => {
      const n = int(next, 6, 12);
      const ops: [string, number, number?][] = [];
      const k = int(next, 4, 8);
      for (let j = 0; j < k; j++) {
        if (j < 2 || next() < 0.55) ops.push(["add", int(next, 0, n - 1), int(next, 1, 9)]);
        else ops.push(["sum", int(next, 0, n - 1)]);
      }
      return enc(n, ops);
    },
    "power-of-two": () => enc(8, [["add", 0, 3], ["add", 4, 2], ["add", 6, 5], ["sum", 7], ["sum", 3]]),
    "odd-positions": () => enc(10, [["add", 2, 4], ["add", 6, 1], ["add", 9, 7], ["sum", 6], ["sum", 8], ["sum", 9]]),
  },
  markers: MARKERS,
  narrationKeys: KEYS,
  counters: [
    { slot: "comparisons", label: "reads" },
    { slot: "writes", label: "writes" },
  ],
  legend: ["key", "sorted", "compare"],
  roleLabels: { key: "written", sorted: "read", compare: "in_query" },
  vars: ["acc"],
  trace,
};
