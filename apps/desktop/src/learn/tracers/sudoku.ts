// Sudoku by backtracking. Mirrors content/learn/backtracking/sudoku/sample.{py,ts}: take the
// first empty cell (row by row), try the digits in order, keep one that its row, column
// and box do not already hold, and go on; when no digit fits, undo the last guess.
// Output: the solved grid row by row, or [-1] when the givens have no solution. Givens
// are bold; guesses are normal weight.
import { fail } from "../engine/codecs";
import { type InputError } from "../engine/inputs";
import { Recorder } from "../engine/recorder";
import type { NarrationKey, Step, TracerDef } from "../engine/types";

type Key = NarrationKey<"sudoku">;
const KEYS = ["sudoku.ready", "sudoku.place", "sudoku.undo", "sudoku.done", "sudoku.impossible"] as const satisfies readonly Key[];
const say = (key: (typeof KEYS)[number], params?: Record<string, number>) => ({ key, params });

export const MARKERS = ["place", "undo"] as const;

function trace(input: readonly number[]): Step[] {
  const size = input[0]!;
  const box = Math.round(Math.sqrt(size));
  const grid = input.slice(1);
  const givens = grid.flatMap((v, k) => (v ? [k] : []));
  const r = new Recorder([]);
  const snap = (narration: ReturnType<typeof say>, marker: string | null, hot: number | null = null, role: "key" | "out" = "key") =>
    r.snap({
      narration,
      marker,
      board: { rows: size, cols: size, cells: grid.map((v) => (v ? String(v) : "")), roles: hot === null ? {} : { [hot]: role }, given: givens, boxes: box },
      vars: { empty: grid.filter((v) => v === 0).length },
    });
  const ok = (at: number, d: number) => {
    const row = Math.floor(at / size);
    const col = at % size;
    for (let k = 0; k < size; k++) if (grid[row * size + k] === d || grid[k * size + col] === d) return false;
    const top = row - (row % box);
    const left = col - (col % box);
    for (let rr = top; rr < top + box; rr++) for (let cc = left; cc < left + box; cc++) if (grid[rr * size + cc] === d) return false;
    return true;
  };
  const fill = (): boolean => {
    const at = grid.indexOf(0);
    if (at === -1) return true;
    for (let d = 1; d <= size; d++) {
      r.comparisons++;
      if (ok(at, d)) {
        grid[at] = d;
        r.moves++;
        snap(say("sudoku.place", { row: Math.floor(at / size), col: at % size, digit: d }), "place", at);
        if (fill()) return true;
        grid[at] = 0;
        r.writes++;
        snap(say("sudoku.undo", { row: Math.floor(at / size), col: at % size, digit: d }), "undo", at, "out");
      }
    }
    return false;
  };

  snap(say("sudoku.ready", { size, empty: grid.filter((v) => v === 0).length }), null);
  const solved = fill();
  snap(solved ? say("sudoku.done", { guesses: r.moves, undos: r.writes }) : say("sudoku.impossible"), null);
  r.steps[r.steps.length - 1]!.result = solved ? [...grid] : [-1];
  return r.steps;
}

/** A map like `1..4/..../..../4..1`: rows by `/`, `.` empty. */
export const encodeSudoku = (map: string) => {
  const rows = map.split("/");
  return [rows.length, ...[...rows.join("")].map((ch) => (ch === "." ? 0 : Number(ch)))];
};

const checkGivens = (input: readonly number[]): InputError | null => {
  const size = input[0]!;
  if (size !== 4 && size !== 9) return { code: "custom", key: "sudoku_size" };
  if (input.length !== 1 + size * size) return { code: "custom", key: "grid_ragged" };
  const box = Math.round(Math.sqrt(size));
  const g = input.slice(1);
  if (g.some((v) => v < 0 || v > size)) return { code: "custom", key: "sudoku_digit", params: { max: size } };
  for (let a = 0; a < g.length; a++) {
    if (!g[a]) continue;
    for (let b = a + 1; b < g.length; b++) {
      if (g[a] !== g[b]) continue;
      const [ra, ca, rb, cb] = [Math.floor(a / size), a % size, Math.floor(b / size), b % size];
      const sameBox = Math.floor(ra / box) === Math.floor(rb / box) && Math.floor(ca / box) === Math.floor(cb / box);
      if (ra === rb || ca === cb || sameBox) return { code: "custom", key: "sudoku_clash", params: { digit: g[a]! } };
    }
  }
  return null;
};

export const sudoku: TracerDef = {
  id: "sudoku",
  subject: "backtracking",
  scene: "board",
  limits: { min_len: 17, max_len: 82, min: 0, max: 9 },
  input: {
    kind: "codec",
    codec: {
      fields: ["map"],
      format: (input) => {
        const size = input[0] ?? 4;
        return [Array.from({ length: size }, (_, row) => input.slice(1 + row * size, 1 + (row + 1) * size).map((v) => (v ? String(v) : ".")).join("")).join("/")];
      },
      parse: (texts) => {
        const rows = (texts[0] ?? "").split(/[/\n]+/).map((s) => s.trim()).filter((s) => s !== "");
        if (rows.length !== 4 && rows.length !== 9) return fail("sudoku_size");
        if (rows.some((row) => row.length !== rows.length)) return fail("grid_ragged");
        const bad = rows.join("").match(/[^.1-9]/);
        if (bad) return fail("sudoku_digit", { max: rows.length });
        const values = encodeSudoku(rows.join("/"));
        const e = checkGivens(values);
        return e ? { error: e } : { values };
      },
      check: checkGivens,
    },
  },
  presets: {
    "four-by-four": () => encodeSudoku("1.../..2./.3../...4"),
    random: (next) => {
      // A solved 4×4 grid, rows and digits shuffled, with some cells emptied.
      const base = [1, 2, 3, 4, 3, 4, 1, 2, 2, 1, 4, 3, 4, 3, 2, 1];
      const digits = [1, 2, 3, 4];
      for (let k = 3; k > 0; k--) {
        const j = Math.floor(next() * (k + 1));
        [digits[k], digits[j]] = [digits[j]!, digits[k]!];
      }
      return [4, ...base.map((v) => (next() < 0.55 ? 0 : digits[v - 1]!))];
    },
    "nine-by-nine": () =>
      encodeSudoku(".52.48.79/197..5.../..37196.5/....7.2../26893..../.15826.94/...6....2/57...291./..4.9.5.7"),
  },
  markers: MARKERS,
  narrationKeys: KEYS,
  counters: [
    { slot: "comparisons", label: "comparisons" },
    { slot: "moves", label: "placed" },
    { slot: "writes", label: "undone" },
  ],
  legend: ["key", "out"],
  roleLabels: { key: "current", out: "removed" },
  vars: ["empty"],
  trace,
};
