// Lowest common ancestor by binary lifting. Mirrors
// content/learn/trees/lca-binary-lifting/sample.{py,ts}: up[k][v] is v's 2^k-th ancestor
// (the root is its own parent), filled level by level as up[k-1] of up[k-1]. For a pair,
// lift the deeper node by the depth difference, one power of two per set bit; if both
// meet, that is the answer; otherwise jump both by every power, largest first, that keeps
// them apart; their parent is the answer. Input: parents (node k+1's parent, < k+1), then
// the pairs. Output: one ancestor per pair.
import { fail } from "../engine/codecs";
import { int, tokens, type InputError } from "../engine/inputs";
import { Recorder } from "../engine/recorder";
import type { ForestNode, Item, NarrationKey, Role, Step, TracerDef } from "../engine/types";

type Key = NarrationKey<"lca-binary-lifting">;
const KEYS = [
  "lca-binary-lifting.ready",
  "lca-binary-lifting.table",
  "lca-binary-lifting.pair",
  "lca-binary-lifting.lift",
  "lca-binary-lifting.same",
  "lca-binary-lifting.jump",
  "lca-binary-lifting.answer",
  "lca-binary-lifting.done",
] as const satisfies readonly Key[];
const say = (key: (typeof KEYS)[number], params?: Record<string, number>) => ({ key, params });

export const MARKERS = ["table", "lift", "same", "jump", "answer"] as const;
export const MAX_NODES = 12;
export const MAX_PAIRS = 4;

export const split = (input: readonly number[]) => {
  const m = input[0] ?? 0;
  const parents = input.slice(1, 1 + m);
  const flat = input.slice(1 + m);
  const pairs = Array.from({ length: Math.floor(flat.length / 2) }, (_, k) => [flat[2 * k]!, flat[2 * k + 1]!] as [number, number]);
  return { parents, pairs, odd: flat.length % 2 === 1 };
};
export const enc = (parents: number[], pairs: [number, number][]) => [parents.length, ...parents, ...pairs.flat()];

function trace(input: readonly number[]): Step[] {
  const { parents, pairs } = split(input);
  const n = parents.length + 1;
  const levels = Math.max(1, n.toString(2).length);
  const r = new Recorder([]);
  const up = Array.from({ length: levels }, () => new Array<number>(n).fill(0));
  const depth = new Array<number>(n).fill(0);
  for (let v = 1; v < n; v++) {
    up[0]![v] = parents[v - 1]!;
    depth[v] = depth[parents[v - 1]!]! + 1;
  }
  const out: Item[] = [];
  const snap = (narration: ReturnType<typeof say>, marker: string | null, roles: Record<number, Role> = {}, marks: readonly number[] = [], vars: Record<string, number | null> = {}) => {
    const forest: ForestNode[] = Array.from({ length: n }, (_, v) => ({
      id: v,
      parent: v === 0 ? null : parents[v - 1]!,
      text: String(v),
      role: roles[v],
      note: `d${depth[v]}`,
      mark: marks.includes(v),
    }));
    r.snap({ narration, marker, forest, aux: { label: "found", cells: out.map((c) => ({ ...c })), roles: {} }, vars: { a: null, b: null, k: null, ...vars } });
  };

  snap(say("lca-binary-lifting.ready", { n, levels }), null);
  for (let k = 1; k < levels; k++) {
    for (let v = 0; v < n; v++) {
      up[k]![v] = up[k - 1]![up[k - 1]![v]!]!;
      r.writes++;
      snap(say("lca-binary-lifting.table", { v, k, jump: 2 ** k, to: up[k]![v]!, mid: up[k - 1]![v]! }), "table", { [v]: "key", [up[k - 1]![v]!]: "compare", [up[k]![v]!]: "sorted" }, [], { k });
    }
  }
  for (const [a0, b0] of pairs) {
    let a = a0;
    let b = b0;
    snap(say("lca-binary-lifting.pair", { a, b }), null, { [a]: "pivot", [b]: "pivot" }, [], { a, b });
    if (depth[a]! < depth[b]!) [a, b] = [b, a];
    const diff = depth[a]! - depth[b]!;
    for (let k = 0; k < levels; k++) {
      if ((diff >> k) & 1) {
        const from = a;
        a = up[k]![a]!;
        r.moves++;
        snap(say("lca-binary-lifting.lift", { from, to: a, jump: 2 ** k }), "lift", { [from]: "compare", [a]: "pivot", [b]: "pivot" }, [], { a, b, k });
      }
    }
    if (a === b) {
      out.push({ id: out.length, value: a });
      snap(say("lca-binary-lifting.same", { a: a0, b: b0, lca: a }), "same", { [a]: "sorted" }, [a], { a, b });
      continue;
    }
    for (let k = levels - 1; k >= 0; k--) {
      r.comparisons++;
      if (up[k]![a] !== up[k]![b]) {
        const fa = a;
        const fb = b;
        a = up[k]![a]!;
        b = up[k]![b]!;
        r.moves++;
        snap(say("lca-binary-lifting.jump", { from_a: fa, from_b: fb, a, b, jump: 2 ** k }), "jump", { [fa]: "compare", [fb]: "compare", [a]: "pivot", [b]: "pivot" }, [], { a, b, k });
      }
    }
    const lca = up[0]![a]!;
    out.push({ id: out.length, value: lca });
    snap(say("lca-binary-lifting.answer", { a: a0, b: b0, lca }), "answer", { [a]: "pivot", [b]: "pivot", [lca]: "sorted" }, [lca], { a, b });
  }
  snap(say("lca-binary-lifting.done", { pairs: pairs.length, jumps: r.moves }), null);
  r.steps[r.steps.length - 1]!.result = out.map((c) => c.value);
  return r.steps;
}

export const lcaBinaryLifting: TracerDef = {
  id: "lca-binary-lifting",
  subject: "trees",
  scene: "tree",
  limits: { min_len: 4, max_len: 20, min: 0, max: 11 },
  input: {
    kind: "codec",
    codec: {
      fields: ["parents", "pairs"],
      format: (input) => {
        const { parents, pairs } = split(input);
        return [parents.join(", "), pairs.map(([a, b]) => `${a} ${b}`).join(", ")];
      },
      parse: (texts) => {
        const p = tokens(texts[0] ?? "");
        if ("error" in p) return p;
        const q = tokens(texts[1] ?? "");
        if ("error" in q) return q;
        if (q.values.length % 2 === 1) return fail("pairs_even");
        return { values: [p.values.length, ...p.values, ...q.values] };
      },
      check: (input): InputError | null => {
        const { parents, pairs, odd } = split(input);
        const n = parents.length + 1;
        if (n < 2 || n > MAX_NODES) return { code: "custom", key: "node_count", params: { min: 2, max: MAX_NODES } };
        const bad = parents.findIndex((p, k) => p < 0 || p > k);
        if (bad >= 0) return { code: "custom", key: "parent", params: { node: bad + 1, max: bad } };
        if (odd) return { code: "custom", key: "pairs_even" };
        if (pairs.length < 1 || pairs.length > MAX_PAIRS) return { code: "custom", key: "pair_count", params: { min: 1, max: MAX_PAIRS } };
        if (pairs.some(([a, b]) => a >= n || b >= n)) return { code: "custom", key: "node_range", params: { max: n - 1 } };
        return null;
      },
    },
  },
  presets: {
    random: (next) => {
      const n = int(next, 7, MAX_NODES);
      const parents = Array.from({ length: n - 1 }, (_, k) => int(next, Math.max(0, k - 3), k));
      const pairs = Array.from({ length: int(next, 2, MAX_PAIRS) }, () => [int(next, 0, n - 1), int(next, 0, n - 1)] as [number, number]);
      return enc(parents, pairs);
    },
    "deep-path": () => enc([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10], [[11, 3], [7, 9]]),
    "two-branches": () => enc([0, 1, 2, 3, 0, 5, 6, 7, 1, 9], [[4, 8], [10, 3], [6, 6]]),
  },
  markers: MARKERS,
  narrationKeys: KEYS,
  counters: [
    { slot: "writes", label: "table_cells" },
    { slot: "moves", label: "jumps" },
    { slot: "comparisons", label: "comparisons" },
  ],
  legend: ["key", "compare", "pivot", "sorted"],
  roleLabels: { key: "current", compare: "passed", pivot: "climbing", sorted: "answer" },
  vars: ["a", "b", "k"],
  trace,
};
