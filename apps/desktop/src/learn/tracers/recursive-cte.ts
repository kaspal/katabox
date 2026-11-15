// A recursive CTE: everyone's depth in a management chain. Mirrors
// content/learn/databases/recursive-cte/sample.{py,ts}:
// WITH RECURSIVE chain(id, depth) AS (
//   SELECT id, 0 FROM emp WHERE boss IS NULL            -- the anchor
//   UNION ALL
//   SELECT e.id, c.depth + 1 FROM emp e JOIN chain c ON e.boss = c.id  -- the recursive part
// ) SELECT depth FROM chain ORDER BY id.
// The anchor gives the first rows; each round joins only the rows found in the round before
// (the "working table") with the employees, until a round finds nothing.
// Output: each employee's depth, by id.
import { bossesCodec } from "../engine/shapes";
import { Recorder } from "../engine/recorder";
import type { NarrationKey, NarrationParams, Role, Step, TracerDef } from "../engine/types";

type Key = NarrationKey<"recursive-cte">;
const KEYS = ["recursive-cte.ready", "recursive-cte.anchor", "recursive-cte.round", "recursive-cte.join", "recursive-cte.done"] as const satisfies readonly Key[];
const say = (key: (typeof KEYS)[number], params?: NarrationParams<number>) => ({ key, params });

export const MARKERS = ["anchor", "join"] as const;

function trace(input: readonly number[]): Step[] {
  const boss = [...input];
  const n = boss.length;
  const r = new Recorder([]);
  const depth: (number | null)[] = Array(n).fill(null);
  const empRoles: Record<number, Role> = {};
  const chain: { id: number; depth: number; round: number }[] = [];
  let round = 0;
  let hot: number | null = null;
  const snap = (narration: ReturnType<typeof say>, marker: string | null) =>
    r.snap({
      narration,
      marker,
      relations: {
        tables: [{ label: "employees", columns: ["id", "boss"], rows: boss.map((b, i) => ({ id: i, cells: [i, b === -1 ? null : b], role: empRoles[i] })) }],
        result: {
          label: "chain",
          columns: ["id", "depth"],
          rows: chain.map((c, k) => ({ id: k, cells: [c.id, c.depth], role: c.id === hot ? ("swap" as Role) : c.round === round - 1 ? ("compare" as Role) : ("sorted" as Role) })),
        },
      },
      vars: { round, rows: chain.length },
    });

  snap(say("recursive-cte.ready", { n }), null);
  let frontier: number[] = [];
  for (let i = 0; i < n; i++) {
    r.comparisons++;
    if (boss[i] === -1) {
      depth[i] = 0;
      chain.push({ id: i, depth: 0, round: 0 });
      frontier.push(i);
      r.writes++;
      empRoles[i] = "sorted";
      hot = i;
      snap(say("recursive-cte.anchor", { id: i }), "anchor");
    }
  }
  hot = null;
  round = 1;
  while (frontier.length > 0) {
    r.moves++;
    snap(say("recursive-cte.round", { round, rows: frontier.length }), null);
    const next: number[] = [];
    for (const b of frontier) {
      for (let e = 0; e < n; e++) {
        r.comparisons++;
        if (boss[e] === b) {
          depth[e] = depth[b]! + 1;
          chain.push({ id: e, depth: depth[e]!, round });
          next.push(e);
          r.writes++;
          empRoles[e] = "sorted";
          hot = e;
          snap(say("recursive-cte.join", { id: e, boss: b, depth: depth[e]! }), "join");
        }
      }
    }
    hot = null;
    frontier = next;
    round++;
  }
  snap(say("recursive-cte.done", { n, rounds: round - 1 }), null);
  r.steps[r.steps.length - 1]!.result = depth.map((d) => d ?? -1);
  return r.steps;
}

export const recursiveCte: TracerDef = {
  id: "recursive-cte",
  subject: "databases",
  scene: "relations",
  limits: { min_len: 2, max_len: 10, min: -1, max: 9 },
  input: { kind: "codec", codec: bossesCodec({ count: [2, 10] }) },
  presets: {
    "small-company": () => [-1, 0, 0, 1, 1, 2],
    "long-chain": () => [-1, 0, 1, 2, 3, 4],
    random: (next) => {
      const n = 4 + Math.floor(next() * 6);
      return Array.from({ length: n }, (_, i) => (i === 0 ? -1 : Math.floor(next() * i)));
    },
  },
  markers: MARKERS,
  narrationKeys: KEYS,
  counters: [
    { slot: "moves", label: "rounds" },
    { slot: "comparisons", label: "comparisons" },
    { slot: "writes", label: "joined" },
  ],
  legend: ["swap", "compare", "sorted"],
  roleLabels: { swap: "written", compare: "reading", sorted: "finished" },
  vars: ["round", "rows"],
  trace,
};
