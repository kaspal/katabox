// Tree diameter by two BFS runs. Mirrors content/learn/trees/tree-diameter/sample.{py,ts}:
// a BFS from any node (0) ends at a node that is one end of a longest path; a second BFS
// from there reaches the other end, and its distance is the diameter (in edges). The input
// gives each node's parent (node k's parent is smaller than k), so it is always a tree.
import { int, tokens } from "../engine/inputs";
import { Recorder } from "../engine/recorder";
import type { ForestNode, NarrationKey, Role, Step, TracerDef } from "../engine/types";

type Key = NarrationKey<"tree-diameter">;
const KEYS = [
  "tree-diameter.ready",
  "tree-diameter.first_bfs",
  "tree-diameter.visit",
  "tree-diameter.reach",
  "tree-diameter.second_bfs",
  "tree-diameter.done",
] as const satisfies readonly Key[];
const say = (key: (typeof KEYS)[number], params?: Record<string, number>) => ({ key, params });

export const MARKERS = ["first-bfs", "visit", "reach", "second-bfs"] as const;

function trace(parents: readonly number[]): Step[] {
  const n = parents.length + 1;
  const adj: number[][] = Array.from({ length: n }, () => []);
  parents.forEach((p, k) => {
    adj[p]!.push(k + 1);
    adj[k + 1]!.push(p);
  });
  const r = new Recorder([]);
  let dist: number[] = new Array<number>(n).fill(-1);
  let best = 0;
  const snap = (narration: ReturnType<typeof say>, marker: string | null, roles: Record<number, Role> = {}) => {
    const forest: ForestNode[] = Array.from({ length: n }, (_, k) => ({
      id: k,
      parent: k === 0 ? null : parents[k - 1]!,
      text: String(k),
      role: roles[k] ?? (dist[k]! >= 0 ? "sorted" : undefined),
      ...(dist[k]! >= 0 ? { note: `d=${dist[k]}` } : {}),
    }));
    r.snap({ narration, marker, forest, vars: { best, farthest: dist[best]! >= 0 ? dist[best]! : null } });
  };
  const farthest = (start: number) => {
    dist = new Array<number>(n).fill(-1);
    dist[start] = 0;
    const queue = [start];
    best = start;
    while (queue.length > 0) {
      const node = queue.shift()!;
      if (dist[node]! > dist[best]!) best = node;
      r.comparisons++;
      snap(say("tree-diameter.visit", { node, d: dist[node]!, best }), "visit", { [node]: "key", [best]: "pivot" });
      for (const nxt of adj[node]!) {
        if (dist[nxt] === -1) {
          dist[nxt] = dist[node]! + 1;
          queue.push(nxt);
          snap(say("tree-diameter.reach", { node: nxt, d: dist[nxt]! }), "reach", { [node]: "key", [nxt]: "swap" });
        }
      }
    }
    return best;
  };
  snap(say("tree-diameter.ready", { n }), null);
  snap(say("tree-diameter.first_bfs"), "first-bfs", { 0: "key" });
  const end = farthest(0);
  const reached = dist[end]!;
  dist = new Array<number>(n).fill(-1);
  snap(say("tree-diameter.second_bfs", { end, d: reached }), "second-bfs", { [end]: "key" });
  const other = farthest(end);
  const length = dist[other]!;
  snap(say("tree-diameter.done", { a: end, b: other, length }), null, { [end]: "pivot", [other]: "pivot" });
  r.steps[r.steps.length - 1]!.result = [length];
  return r.steps;
}

/** A random tree as parents: node k hangs under a random earlier node. */
const randomParents = (next: () => number, n: number) => Array.from({ length: n - 1 }, (_, k) => int(next, 0, k));

export const treeDiameter: TracerDef = {
  id: "tree-diameter",
  subject: "trees",
  scene: "tree",
  limits: { min_len: 1, max_len: 13, min: 0, max: 12 },
  input: {
    kind: "codec",
    codec: {
      fields: ["parents"],
      format: (input) => [input.join(", ")],
      parse: (texts) => tokens(texts[0] ?? ""),
      check: (input) => {
        const bad = input.findIndex((p, k) => p < 0 || p > k);
        return bad >= 0 ? { code: "custom", key: "parent", params: { node: bad + 1, max: bad } } : null;
      },
    },
  },
  presets: {
    random: (next) => randomParents(next, int(next, 6, 12)),
    path: () => [0, 1, 2, 3, 4, 5],
    star: () => [0, 0, 0, 0, 0, 0],
    caterpillar: () => [0, 1, 2, 3, 1, 2, 3, 0],
  },
  markers: MARKERS,
  narrationKeys: KEYS,
  counters: [{ slot: "comparisons", label: "visits" }],
  legend: ["key", "swap", "pivot", "sorted"],
  roleLabels: { key: "visiting", swap: "reached", pivot: "farthest_so_far", sorted: "visited" },
  vars: ["best", "farthest"],
  trace,
};
