// Huffman coding. Mirrors content/learn/greedy/huffman-coding/sample.{py,ts}: every
// symbol starts as a one-node tree weighted by its frequency; repeatedly take the two
// lightest trees (ties: the older node first) and join them under a new node. A
// symbol's code length is its depth in the final tree (left edge 0, right edge 1).
// Output: the total encoded length in bits (Σ frequency × depth), then each symbol's
// code length.
import { Recorder } from "../engine/recorder";
import type { ForestNode, NarrationKey, Role, Step, TracerDef } from "../engine/types";

type Key = NarrationKey<"huffman-coding">;
const KEYS = ["huffman-coding.ready", "huffman-coding.merge", "huffman-coding.done"] as const satisfies readonly Key[];
const say = (key: (typeof KEYS)[number], params?: Record<string, number | string>) => ({ key, params: params as Record<string, number> });

export const MARKERS = ["merge"] as const;
const LETTERS = "abcdefghijklmnop";

function trace(freqs: readonly number[]): Step[] {
  const n = freqs.length;
  const r = new Recorder([]);
  const weight = [...freqs];
  const parent = new Map<number, number>();
  const kids = new Map<number, [number, number]>();
  let queue = freqs.map((f, k) => [f, k] as [number, number]).sort((x, y) => x[0] - y[0] || x[1] - y[1]);
  let made = n;
  const forest = (roles: Record<number, Role>): ForestNode[] => {
    const out: ForestNode[] = [];
    const walk = (id: number, p: number | null, side?: "L" | "R") => {
      out.push({ id, parent: p, text: String(weight[id]), role: roles[id], side, edge: side ? (side === "L" ? "0" : "1") : undefined, note: id < n ? LETTERS[id] : undefined });
      const c = kids.get(id);
      if (c) {
        walk(c[0], id, "L");
        walk(c[1], id, "R");
      }
    };
    // Roots in queue order, so the forest reads lightest first.
    for (const [, id] of queue) walk(id, null);
    return out;
  };
  const snap = (narration: ReturnType<typeof say>, marker: string | null, roles: Record<number, Role> = {}) =>
    r.snap({
      narration,
      marker,
      forest: forest(roles),
      slots: { layout: "row", label: "trees_by_weight", cells: queue.map(([f, id]) => ({ id, value: f })), roles: {} },
      vars: { trees: queue.length },
    });

  snap(say("huffman-coding.ready", { n }), null);
  while (queue.length > 1) {
    const [fa, a] = queue[0]!;
    const [fb, b] = queue[1]!;
    queue = queue.slice(2);
    parent.set(a, made);
    parent.set(b, made);
    kids.set(made, [a, b]);
    weight[made] = fa + fb;
    queue.push([fa + fb, made]);
    queue.sort((x, y) => x[0] - y[0] || x[1] - y[1]);
    r.writes++;
    snap(say("huffman-coding.merge", { a: fa, b: fb, sum: fa + fb }), "merge", { [made]: "key", [a]: "compare", [b]: "compare" });
    made++;
  }
  const lengths = freqs.map((_, k) => {
    let depth = 0;
    for (let node = k; parent.has(node); node = parent.get(node)!) depth++;
    return depth;
  });
  const bits = freqs.reduce((s, f, k) => s + f * lengths[k]!, 0);
  snap(say("huffman-coding.done", { bits, fixed: freqs.reduce((s, f) => s + f, 0) * Math.max(1, Math.ceil(Math.log2(n))) }), null);
  r.steps[r.steps.length - 1]!.result = [bits, ...lengths];
  return r.steps;
}

export const huffmanCoding: TracerDef = {
  id: "huffman-coding",
  subject: "greedy",
  scene: "tree",
  limits: { min_len: 2, max_len: 8, min: 1, max: 40 },
  input: { kind: "values" },
  presets: {
    skewed: () => [4, 7, 11, 15, 18, 40],
    random: (next) => Array.from({ length: 3 + Math.floor(next() * 6) }, () => 1 + Math.floor(next() * 40)),
    even: () => [10, 10, 10, 10],
  },
  markers: MARKERS,
  narrationKeys: KEYS,
  counters: [{ slot: "writes", label: "merges" }],
  legend: ["key", "compare"],
  roleLabels: { key: "new_node", compare: "merged_pair" },
  vars: ["trees"],
  trace,
};
