// Union-find (disjoint sets). Mirrors content/learn/trees/union-find/sample.{py,ts}: each
// set is a tree of parent pointers; find climbs to the root, then points every node on the
// way straight at it (path compression); union links the smaller tree's root under the
// bigger one (union by size). Output: the find answers, then how many sets are left.
import { decodeOps, opsCodec, type OpSpec } from "../engine/codecs";
import { int } from "../engine/inputs";
import { Recorder } from "../engine/recorder";
import type { ForestNode, Item, NarrationKey, Role, Step, TracerDef } from "../engine/types";

type Key = NarrationKey<"union-find">;
const KEYS = [
  "union-find.ready",
  "union-find.climb",
  "union-find.compress",
  "union-find.root",
  "union-find.same",
  "union-find.link",
  "union-find.done",
] as const satisfies readonly Key[];
const say = (key: (typeof KEYS)[number], params?: Record<string, number>) => ({ key, params });

export const MARKERS = ["climb", "compress", "same", "link"] as const;
export const OPS: OpSpec[] = [
  { name: "union", args: 2 },
  { name: "find", args: 1 },
];
const codec = opsCodec(OPS, ["count"]);
const enc = (n: number, ops: [string, number, number?][]) => [n, ...ops.flatMap(([name, a, b]) => [OPS.findIndex((o) => o.name === name) + 1, a, b ?? 0])];

function trace(input: readonly number[]): Step[] {
  const n = input[0]!;
  const ops = decodeOps(input, OPS, codec.width, 1);
  const r = new Recorder([]);
  const parent = Array.from({ length: n }, (_, k) => k);
  const size = new Array<number>(n).fill(1);
  const out: Item[] = [];
  const snap = (narration: ReturnType<typeof say>, marker: string | null, roles: Record<number, Role> = {}, outRole = false) => {
    const forest: ForestNode[] = parent.map((p, k) => ({
      id: k,
      parent: p === k ? null : p,
      text: String(k),
      role: roles[k],
      mark: p === k,
      ...(p === k ? { note: `×${size[k]}` } : {}),
    }));
    r.snap({
      narration,
      marker,
      forest,
      aux: { label: "found", cells: out.map((c) => ({ ...c })), roles: outRole ? { [out.length - 1]: "swap" } : {} },
      vars: { sets: parent.filter((p, k) => p === k).length },
    });
  };
  const find = (x: number): number => {
    let root = x;
    while (parent[root] !== root) {
      root = parent[root]!;
      r.comparisons++;
      snap(say("union-find.climb", { x, at: root }), "climb", { [x]: "key", [root]: "compare" });
    }
    while (parent[x] !== root) {
      const next = parent[x]!;
      parent[x] = root;
      r.writes++;
      snap(say("union-find.compress", { node: x, root }), "compress", { [x]: "swap", [root]: "sorted" });
      x = next;
    }
    return root;
  };

  snap(say("union-find.ready", { n }), null);
  for (const op of ops) {
    if (op.name === "find") {
      const x = op.args[0]!;
      const root = find(x);
      out.push({ id: out.length, value: root });
      snap(say("union-find.root", { x, root }), null, { [root]: "sorted" }, true);
    } else {
      const [a, b] = op.args as [number, number];
      let ra = find(a);
      let rb = find(b);
      if (ra === rb) {
        snap(say("union-find.same", { a, b, root: ra }), "same", { [ra]: "sorted" });
        continue;
      }
      if (size[ra]! < size[rb]!) [ra, rb] = [rb, ra];
      parent[rb] = ra;
      size[ra]! += size[rb]!;
      r.moves++;
      snap(say("union-find.link", { child: rb, root: ra, size: size[ra]! }), "link", { [rb]: "swap", [ra]: "sorted" });
    }
  }
  const sets = parent.filter((p, k) => p === k).length;
  snap(say("union-find.done", { sets }), null);
  r.steps[r.steps.length - 1]!.result = [...out.map((c) => c.value), sets];
  return r.steps;
}

export const unionFind: TracerDef = {
  id: "union-find",
  subject: "trees",
  scene: "tree",
  limits: { min_len: 4, max_len: 37, min: 0, max: 10 },
  input: {
    kind: "codec",
    codec: {
      ...codec,
      check: (input) => {
        const e = codec.check!(input);
        if (e) return e;
        const n = input[0]!;
        if (n < 2 || n > 10) return { code: "custom", key: "node_count", params: { min: 2, max: 10 } };
        for (const op of decodeOps(input, OPS, codec.width, 1)) {
          if (op.args.some((v) => v >= n)) return { code: "custom", key: "node_range", params: { max: n - 1 } };
        }
        return null;
      },
    },
  },
  presets: {
    random: (next) => {
      const n = int(next, 5, 8);
      const ops: [string, number, number?][] = [];
      const k = int(next, 6, 10);
      for (let j = 0; j < k; j++) {
        if (next() < 0.65) ops.push(["union", int(next, 0, n - 1), int(next, 0, n - 1)]);
        else ops.push(["find", int(next, 0, n - 1)]);
      }
      return enc(n, ops);
    },
    chain: () => enc(6, [["union", 0, 1], ["union", 2, 3], ["union", 1, 3], ["union", 4, 5], ["union", 3, 5], ["find", 5], ["find", 4]]),
    "already-joined": () => enc(5, [["union", 0, 1], ["union", 1, 2], ["union", 0, 2], ["find", 2], ["union", 3, 4]]),
  },
  markers: MARKERS,
  narrationKeys: KEYS,
  counters: [
    { slot: "comparisons", label: "climbs" },
    { slot: "writes", label: "compressed" },
    { slot: "moves", label: "links" },
  ],
  legend: ["key", "compare", "swap", "sorted"],
  roleLabels: { key: "start", compare: "climbing", swap: "relinked", sorted: "root" },
  vars: ["sets"],
  trace,
};
