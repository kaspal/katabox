// A hash join. Mirrors content/learn/databases/join-algorithms/sample.{py,ts}:
// SELECT L.key, L.a, R.b FROM L JOIN R ON L.key = R.key. Build: put every R row in a hash
// index by key. Probe: for each L row, look its key up and output one row per match. A
// nested-loop join would compare every L row with every R row instead (the theory tab).
// Output: the joined rows [key, a, b, …] in L order, then R order.
import { rowsCodec } from "../engine/shapes";
import { Recorder } from "../engine/recorder";
import type { NarrationKey, NarrationParams, Relation, Role, Step, TracerDef } from "../engine/types";

type Key = NarrationKey<"join-algorithms">;
const KEYS = [
  "join-algorithms.ready",
  "join-algorithms.build",
  "join-algorithms.probe",
  "join-algorithms.match",
  "join-algorithms.done",
] as const satisfies readonly Key[];
const say = (key: (typeof KEYS)[number], params?: NarrationParams) => ({ key, params: params as NarrationParams<number> });

export const MARKERS = ["build", "probe", "match"] as const;

/** [n_left, …left (key, a), …right (key, b)] as the two tables. */
export function decodeTwoTables(input: readonly number[]): [number, number][][] {
  const n = input[0] ?? 0;
  const pairs = (xs: readonly number[]) => Array.from({ length: Math.floor(xs.length / 2) }, (_, k) => [xs[2 * k]!, xs[2 * k + 1]!] as [number, number]);
  return [pairs(input.slice(1, 1 + 2 * n)), pairs(input.slice(1 + 2 * n))];
}

const encode = (left: [number, number][], right: [number, number][]) => [left.length, ...left.flat(), ...right.flat()];

function trace(input: readonly number[]): Step[] {
  const [left, right] = decodeTwoTables(input);
  const r = new Recorder([]);
  const leftRoles: Record<number, Role> = {};
  const rightRoles: Record<number, Role> = {};
  const index = new Map<number, number[]>(); // key → R row positions
  const out: [number, number, number][] = [];
  let lastOut = -1;
  const snap = (narration: ReturnType<typeof say>, marker: string | null, hotKey: number | null = null) => {
    const tables: Relation[] = [
      { label: "left_table", columns: ["key", "a"], rows: left.map(([k, a], i) => ({ id: i, cells: [k, a], role: leftRoles[i] })) },
      { label: "right_table", columns: ["key", "b"], rows: right.map(([k, b], i) => ({ id: i, cells: [k, b], role: rightRoles[i] })) },
      {
        label: "hash_index",
        columns: ["key", "R rows"],
        rows: [...index.entries()].map(([k, ids]) => ({ id: k, cells: [k, ids.map((i) => `#${i + 1}`).join(" ")], role: k === hotKey ? ("compare" as Role) : undefined })),
      },
    ];
    r.snap({
      narration,
      marker,
      relations: {
        tables,
        result: { label: "joined", columns: ["key", "a", "b"], rows: out.map((row, i) => ({ id: i, cells: [...row], role: i === lastOut ? ("swap" as Role) : ("sorted" as Role) })) },
      },
      vars: { rows_out: out.length, index_keys: index.size },
    });
  };

  snap(say("join-algorithms.ready", { left: left.length, right: right.length }), null);
  for (const [i, [k]] of right.entries()) {
    const ids = index.get(k) ?? [];
    ids.push(i);
    index.set(k, ids);
    r.moves++;
    rightRoles[i] = "key";
    snap(say("join-algorithms.build", { row: i + 1, key: k }), "build", k);
    rightRoles[i] = "sorted";
  }
  for (const [i, [k, a]] of left.entries()) {
    leftRoles[i] = "key";
    r.comparisons++;
    const matches = index.get(k) ?? [];
    lastOut = -1;
    snap(say("join-algorithms.probe", { row: i + 1, key: k, matches: matches.length }), "probe", k);
    for (const j of matches) {
      out.push([k, a, right[j]![1]]);
      lastOut = out.length - 1;
      r.writes++;
      rightRoles[j] = "compare";
      snap(say("join-algorithms.match", { key: k, a, b: right[j]![1] }), "match", k);
      rightRoles[j] = "sorted";
    }
    lastOut = -1;
    leftRoles[i] = matches.length > 0 ? "sorted" : "out";
  }
  snap(say("join-algorithms.done", { rows: out.length, pairs: left.length * right.length }), null);
  r.steps[r.steps.length - 1]!.result = out.flat();
  return r.steps;
}

export const joinAlgorithms: TracerDef = {
  id: "join-algorithms",
  subject: "databases",
  scene: "relations",
  limits: { min_len: 5, max_len: 25, min: 0, max: 99 },
  input: { kind: "codec", codec: rowsCodec({ fields: ["left_rows", "right_rows"], a: [1, 6], b: [0, 99], count: [1, 6] }) },
  presets: {
    "one-to-many": () =>
      encode(
        [
          [1, 10],
          [2, 20],
          [3, 30],
        ],
        [
          [2, 5],
          [3, 7],
          [3, 8],
          [4, 9],
        ],
      ),
    "no-match": () =>
      encode(
        [
          [1, 11],
          [2, 12],
        ],
        [
          [5, 1],
          [6, 2],
        ],
      ),
    random: (next) => {
      const table = () => Array.from({ length: 2 + Math.floor(next() * 4) }, () => [1 + Math.floor(next() * 4), Math.floor(next() * 100)] as [number, number]);
      return encode(table(), table());
    },
  },
  markers: MARKERS,
  narrationKeys: KEYS,
  counters: [
    { slot: "moves", label: "inserts" },
    { slot: "comparisons", label: "probes" },
    { slot: "writes", label: "joined" },
  ],
  legend: ["key", "compare", "swap", "sorted", "out"],
  roleLabels: { key: "current", compare: "match", swap: "written", sorted: "finished", out: "skipped" },
  vars: ["rows_out", "index_keys"],
  trace,
};
