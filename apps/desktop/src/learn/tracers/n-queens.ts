// N-Queens by backtracking. Mirrors content/learn/backtracking/n-queens/sample.{py,ts}:
// one queen per row, top to bottom; a column is tried only if no queen above shares its
// column or a diagonal (the hatched cells are the ones under attack); a full board is a
// solution; then the last queen is removed and the next column tried. Every solution is
// counted. Output: the number of solutions, then the first one's columns, row by row.
import { Recorder } from "../engine/recorder";
import type { NarrationKey, NarrationParams, Role, Step, TracerDef } from "../engine/types";
import { Outline } from "./outline";

type Key = NarrationKey<"n-queens">;
const KEYS = ["n-queens.ready", "n-queens.place", "n-queens.solution", "n-queens.dead_end", "n-queens.remove", "n-queens.done"] as const satisfies readonly Key[];
const say = (key: (typeof KEYS)[number], params?: NarrationParams<number>) => ({ key, params });

export const MARKERS = ["place", "solution", "remove"] as const;

function trace(input: readonly number[]): Step[] {
  const n = input[0]!;
  const r = new Recorder([]);
  const outline = new Outline();
  const cols: number[] = [];
  const found: number[][] = [];
  const attacked = () => {
    const out: number[] = [];
    for (let row = 0; row < n; row++)
      for (let c = 0; c < n; c++)
        if (cols.some((cc, rr) => rr !== row && (cc === c || Math.abs(cc - c) === Math.abs(rr - row))) && cols[row] !== c) out.push(row * n + c);
    return out;
  };
  const snap = (narration: ReturnType<typeof say>, marker: string | null, hot: number | null = null, role: Role = "key") => {
    const roles: Record<number, Role> = {};
    if (hot !== null) roles[hot] = role;
    r.snap({
      narration,
      marker,
      board: { rows: n, cols: n, cells: Array.from({ length: n * n }, (_, k) => (cols[Math.floor(k / n)] === k % n ? "Q" : "")), roles, shade: attacked(), checker: true },
      decision: outline.data(),
      vars: { row: cols.length, solutions: found.length },
    });
  };
  const safe = (c: number) => {
    const row = cols.length;
    return cols.every((cc, rr) => c !== cc && Math.abs(c - cc) !== row - rr);
  };
  const place = (line: number) => {
    if (cols.length === n) {
      found.push([...cols]);
      outline.mark(line, "sorted");
      r.writes++;
      snap(say("n-queens.solution", { num: found.length }), "solution");
      return;
    }
    let tried = 0;
    for (let c = 0; c < n; c++) {
      r.comparisons++;
      if (safe(c)) {
        tried++;
        cols.push(c);
        r.moves++;
        const child = outline.open(cols.length, `${cols.length - 1}: ${c}`);
        snap(say("n-queens.place", { row: cols.length - 1, col: c }), "place", (cols.length - 1) * n + c);
        place(child);
        cols.pop();
        outline.back(line);
        snap(say("n-queens.remove", { row: cols.length, col: c }), "remove", cols.length * n + c, "out");
      }
    }
    if (!tried) {
      outline.mark(line, "out");
      snap(say("n-queens.dead_end", { row: cols.length }), null);
    }
  };

  snap(say("n-queens.ready", { n }), null);
  place(outline.open(0, "∅"));
  snap(say("n-queens.done", { n, num: found.length }), null);
  r.steps[r.steps.length - 1]!.result = [found.length, ...(found[0] ?? [])];
  return r.steps;
}

export const nQueens: TracerDef = {
  id: "n-queens",
  subject: "backtracking",
  scene: "board",
  limits: { min_len: 1, max_len: 1, min: 4, max: 6 },
  input: { kind: "count" },
  markers: MARKERS,
  narrationKeys: KEYS,
  counters: [
    { slot: "comparisons", label: "comparisons" },
    { slot: "moves", label: "placed" },
    { slot: "writes", label: "recorded" },
  ],
  legend: ["key", "out", "sorted"],
  roleLabels: { key: "current", out: "removed", sorted: "recorded" },
  vars: ["row", "solutions"],
  trace,
};
