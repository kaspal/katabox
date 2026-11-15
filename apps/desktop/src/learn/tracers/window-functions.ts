// A window function: a running total per partition. Mirrors
// content/learn/databases/window-functions/sample.{py,ts}:
// SELECT p, v, SUM(v) OVER (PARTITION BY p ORDER BY id) FROM t ORDER BY id, with the rows
// already in id order. Unlike GROUP BY, every row stays: each gets the total of its
// partition so far. One pass keeps a running total per partition.
// Output: the running totals, in row order.
import { rowsCodec } from "../engine/shapes";
import { Recorder } from "../engine/recorder";
import type { NarrationKey, NarrationParams, Role, Step, TracerDef } from "../engine/types";

type Key = NarrationKey<"window-functions">;
const KEYS = ["window-functions.ready", "window-functions.start", "window-functions.add", "window-functions.done"] as const satisfies readonly Key[];
const say = (key: (typeof KEYS)[number], params?: NarrationParams<number>) => ({ key, params });

export const MARKERS = ["start", "add"] as const;

const pairs = (xs: readonly number[]) => Array.from({ length: Math.floor(xs.length / 2) }, (_, k) => [xs[2 * k]!, xs[2 * k + 1]!] as [number, number]);

function trace(input: readonly number[]): Step[] {
  const rows = pairs(input);
  const r = new Recorder([]);
  const rowRoles: Record<number, Role> = {};
  const running = new Map<number, number>();
  let hot: number | null = null;
  const out: [number, number, number][] = [];
  const snap = (narration: ReturnType<typeof say>, marker: string | null) =>
    r.snap({
      narration,
      marker,
      relations: {
        tables: [
          { label: "rows_in", columns: ["id", "p", "v"], rows: rows.map(([p, v], i) => ({ id: i, cells: [i + 1, p, v], role: rowRoles[i] })) },
          {
            label: "running_totals",
            columns: ["p", "total"],
            rows: [...running.entries()].map(([p, total]) => ({ id: p, cells: [p, total], role: p === hot ? ("compare" as Role) : undefined })),
          },
        ],
        result: {
          label: "with_totals",
          columns: ["p", "v", "running"],
          rows: out.map((row, i) => ({ id: i, cells: [...row], role: i === out.length - 1 && marker !== null ? ("swap" as Role) : ("sorted" as Role) })),
        },
      },
      vars: { partitions: running.size },
    });

  snap(say("window-functions.ready", { n: rows.length }), null);
  for (const [i, [p, v]] of rows.entries()) {
    rowRoles[i] = "key";
    hot = p;
    r.comparisons++;
    const before = running.get(p);
    if (before === undefined) {
      running.set(p, v);
      r.moves++;
      out.push([p, v, v]);
      r.writes++;
      snap(say("window-functions.start", { p, v }), "start");
    } else {
      running.set(p, before + v);
      out.push([p, v, before + v]);
      r.writes++;
      snap(say("window-functions.add", { p, v, before, total: before + v }), "add");
    }
    rowRoles[i] = "sorted";
  }
  hot = null;
  snap(say("window-functions.done", { n: rows.length, partitions: running.size }), null);
  r.steps[r.steps.length - 1]!.result = out.map((row) => row[2]);
  return r.steps;
}

export const windowFunctions: TracerDef = {
  id: "window-functions",
  subject: "databases",
  scene: "relations",
  limits: { min_len: 2, max_len: 20, min: 0, max: 99 },
  input: { kind: "codec", codec: rowsCodec({ fields: ["rows"], a: [1, 3], b: [0, 99], count: [1, 10] }) },
  presets: {
    "two-accounts": () => [1, 50, 2, 20, 1, 15, 1, 5, 2, 40],
    "one-partition": () => [1, 3, 1, 4, 1, 5, 1, 6],
    random: (next) => Array.from({ length: 4 + Math.floor(next() * 6) }, () => [1 + Math.floor(next() * 3), Math.floor(next() * 50)]).flat(),
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
  vars: ["partitions"],
  trace,
};
