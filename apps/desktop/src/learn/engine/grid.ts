// Grid inputs for the Learn topics (ADR 0029, Phase 6). A grid is stored as one flat
// integer list, `[rows, cols, sr, sc, gr, gc, cell…]` (0 open, 1 wall, 2-9 a cell that
// costs that much to enter; start and goal are open), and typed in "your own" as a map:
// rows separated by `/`, `.` open, `#` wall, `2`-`9` a weight, `S` start, `G` goal, e.g.
// `S..#/.#2./...G`. The Learn driver's `kb_grid` decodes the same list.
import { fail } from "./codecs";
import { int, type InputError } from "./inputs";
import type { GridData, InputCodec, Role } from "./types";

export type Grid = { rows: number; cols: number; start: [number, number]; goal: [number, number]; cells: number[] };

export const MAX_ROWS = 12;
export const MAX_COLS = 16;
/** Up, right, down, left: the order every grid sample tries its neighbours in. */
export const DIRS: readonly [number, number][] = [
  [-1, 0],
  [0, 1],
  [1, 0],
  [0, -1],
];

export function decodeGrid(input: readonly number[]): Grid {
  const [rows = 0, cols = 0, sr = 0, sc = 0, gr = 0, gc = 0] = input;
  return { rows, cols, start: [sr, sc], goal: [gr, gc], cells: input.slice(6) };
}

/** A map such as `S..#/.#2./...G` as a stored grid (presets; no checks). */
export function encodeMap(map: string): number[] {
  const lines = map.split("/");
  const cells: number[] = [];
  let start = [0, 0];
  let goal = [0, 0];
  lines.forEach((line, r) =>
    [...line].forEach((ch, c) => {
      if (ch === "S") start = [r, c];
      if (ch === "G") goal = [r, c];
      cells.push(ch === "#" ? 1 : /[2-9]/.test(ch) ? Number(ch) : 0);
    }),
  );
  return [lines.length, lines[0]!.length, ...start, ...goal, ...cells];
}

export function formatMap(g: Grid): string {
  const rows: string[] = [];
  for (let r = 0; r < g.rows; r++) {
    let line = "";
    for (let c = 0; c < g.cols; c++) {
      const v = g.cells[r * g.cols + c]!;
      line += r === g.start[0] && c === g.start[1] ? "S" : r === g.goal[0] && c === g.goal[1] ? "G" : v === 1 ? "#" : v >= 2 ? String(v) : ".";
    }
    rows.push(line);
  }
  return rows.join("/");
}

export function gridCodec(opts: { weights: boolean }): InputCodec {
  const check = (input: readonly number[]): InputError | null => {
    const g = decodeGrid(input);
    if (g.rows < 2 || g.rows > MAX_ROWS || g.cols < 2 || g.cols > MAX_COLS) return { code: "custom", key: "grid_size", params: { rows: MAX_ROWS, cols: MAX_COLS } };
    if (g.cells.length !== g.rows * g.cols) return { code: "custom", key: "grid_ragged" };
    for (const [r, c] of [g.start, g.goal]) if (r < 0 || r >= g.rows || c < 0 || c >= g.cols) return { code: "custom", key: "grid_start_goal" };
    if (g.start[0] === g.goal[0] && g.start[1] === g.goal[1]) return { code: "custom", key: "grid_start_goal" };
    if (g.cells[g.start[0] * g.cols + g.start[1]] !== 0 || g.cells[g.goal[0] * g.cols + g.goal[1]] !== 0) return { code: "custom", key: "grid_start_goal" };
    if (g.cells.some((v) => v < 0 || v > 9 || (!opts.weights && v > 1))) return { code: "custom", key: opts.weights ? "grid_cell" : "grid_no_weights" };
    return null;
  };
  return {
    fields: ["grid"],
    format: (input) => [formatMap(decodeGrid(input))],
    parse: (texts) => {
      const lines = (texts[0] ?? "")
        .split(/[/\n]+/)
        .map((l) => l.trim())
        .filter((l) => l !== "");
      if (lines.length < 2 || lines.length > MAX_ROWS || lines.some((l) => l.length < 2 || l.length > MAX_COLS)) return fail("grid_size", { rows: MAX_ROWS, cols: MAX_COLS });
      if (lines.some((l) => l.length !== lines[0]!.length)) return fail("grid_ragged");
      const bad = lines.join("").match(opts.weights ? /[^.#SG2-9]/i : /[^.#SG]/i);
      if (bad) return fail(opts.weights ? "grid_cell" : "grid_no_weights", { ch: bad[0] });
      const text = lines.join("/").toUpperCase();
      if ((text.match(/S/g) ?? []).length !== 1 || (text.match(/G/g) ?? []).length !== 1) return fail("grid_start_goal");
      const values = encodeMap(text);
      const e = check(values);
      return e ? { error: e } : { values };
    },
    check,
  };
}

/** A random grid: walls with probability `walls`, weights 2-9 with probability `weights`,
 * start in the top-left area and goal in the bottom-right area. */
export function randomGrid(next: () => number, o: { rows: [number, number]; cols: [number, number]; walls: number; weights?: number }): number[] {
  const rows = int(next, o.rows[0], o.rows[1]);
  const cols = int(next, o.cols[0], o.cols[1]);
  const start: [number, number] = [int(next, 0, Math.floor(rows / 3)), int(next, 0, Math.floor(cols / 3))];
  const goal: [number, number] = [int(next, rows - 1 - Math.floor(rows / 3), rows - 1), int(next, cols - 1 - Math.floor(cols / 3), cols - 1)];
  const cells = Array.from({ length: rows * cols }, () => {
    const x = next();
    return x < o.walls ? 1 : x < o.walls + (o.weights ?? 0) ? int(next, 2, 9) : 0;
  });
  cells[start[0] * cols + start[1]] = 0;
  cells[goal[0] * cols + goal[1]] = 0;
  return [rows, cols, ...start, ...goal, ...cells];
}

/** Scene data: per-cell roles and labels (by cell index r * cols + c), and a path. */
export function gridData(g: Grid, s: { roles?: Record<number, Role>; text?: Record<number, string>; path?: number[] } = {}): GridData {
  return { rows: g.rows, cols: g.cols, cells: [...g.cells], start: g.start, goal: g.goal, roles: s.roles ?? {}, text: s.text ?? {}, path: s.path ?? [] };
}
