// GROUP BY with hash aggregation. Mirrors content/learn/databases/group-by/sample.{py,ts}:
// SELECT g, COUNT(*), SUM(v) FROM t GROUP BY g ORDER BY g. One pass over the rows keeps a
// running (count, sum) per group in a hash table; then the groups come out in key order.
// Output: [g, count, sum, …] by g.
import { rowsCodec } from "../engine/shapes";
import { Recorder } from "../engine/recorder";
import type { NarrationKey, NarrationParams, Role, Step, TracerDef } from "../engine/types";

type Key = NarrationKey<"group-by">;
const KEYS = ["group-by.ready", "group-by.new_group", "group-by.add", "group-by.emit", "group-by.done"] as const satisfies readonly Key[];
const say = (key: (typeof KEYS)[number], params?: NarrationParams<number>) => ({ key, params });

export const MARKERS = ["add", "emit"] as const;

const pairs = (xs: readonly number[]) => Array.from({ length: Math.floor(xs.length / 2) }, (_, k) => [xs[2 * k]!, xs[2 * k + 1]!] as [number, number]);

function trace(input: readonly number[]): Step[] {
  const rows = pairs(input);
  const r = new Recorder([]);
  const rowRoles: Record<number, Role> = {};
  const groups = new Map<number, { count: number; sum: number }>(); // in first-seen order
  let hotGroup: number | null = null;
  const out: [number, number, number][] = [];
  const snap = (narration: ReturnType<typeof say>, marker: string | null) =>
    r.snap({
      narration,
      marker,
      relations: {
        tables: [
          { label: "rows_in", columns: ["g", "v"], rows: rows.map(([g, v], i) => ({ id: i, cells: [g, v], role: rowRoles[i] })) },
          {
            label: "groups",
            columns: ["g", "count", "sum"],
            rows: [...groups.entries()].map(([g, s]) => ({ id: g, cells: [g, s.count, s.sum], role: g === hotGroup ? ("compare" as Role) : undefined })),
          },
        ],
        result: { label: "grouped", columns: ["g", "COUNT(*)", "SUM(v)"], rows: out.map((row, i) => ({ id: i, cells: [...row], role: i === out.length - 1 && marker === "emit" ? ("swap" as Role) : ("sorted" as Role) })) },
      },
      vars: { groups: groups.size },
    });

  snap(say("group-by.ready", { n: rows.length }), null);
  for (const [i, [g, v]] of rows.entries()) {
    rowRoles[i] = "key";
    hotGroup = g;
    const s = groups.get(g);
    r.comparisons++;
    if (s) {
      s.count++;
      s.sum += v;
      snap(say("group-by.add", { g, v, num: s.count, sum: s.sum }), "add");
    } else {
      groups.set(g, { count: 1, sum: v });
      r.moves++;
      snap(say("group-by.new_group", { g, v }), "add");
    }
    rowRoles[i] = "sorted";
  }
  hotGroup = null;
  for (const g of [...groups.keys()].sort((x, y) => x - y)) {
    const s = groups.get(g)!;
    out.push([g, s.count, s.sum]);
    r.writes++;
    hotGroup = g;
    snap(say("group-by.emit", { g, num: s.count, sum: s.sum }), "emit");
  }
  hotGroup = null;
  snap(say("group-by.done", { groups: out.length, n: rows.length }), null);
  r.steps[r.steps.length - 1]!.result = out.flat();
  return r.steps;
}

export const groupBy: TracerDef = {
  id: "group-by",
  subject: "databases",
  scene: "relations",
  limits: { min_len: 2, max_len: 20, min: 0, max: 99 },
  input: { kind: "codec", codec: rowsCodec({ fields: ["rows"], a: [1, 5], b: [0, 99], count: [1, 10] }) },
  presets: {
    "two-groups": () => [1, 10, 2, 5, 1, 30, 2, 7, 1, 2],
    "one-group": () => [3, 4, 3, 9, 3, 1],
    random: (next) => Array.from({ length: 4 + Math.floor(next() * 6) }, () => [1 + Math.floor(next() * 4), Math.floor(next() * 50)]).flat(),
  },
  markers: MARKERS,
  narrationKeys: KEYS,
  counters: [
    { slot: "comparisons", label: "values_read" },
    { slot: "moves", label: "opened" },
    { slot: "writes", label: "stored" },
  ],
  legend: ["key", "compare", "swap", "sorted"],
  roleLabels: { key: "current", compare: "reading", swap: "written", sorted: "finished" },
  vars: ["groups"],
  trace,
};
