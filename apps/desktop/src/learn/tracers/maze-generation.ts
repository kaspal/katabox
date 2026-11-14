// Maze generation by randomized depth-first search. Mirrors
// content/learn/grid-pathfinding/maze-generation/sample.{py,ts}: a (2k+1)-square grid starts as all
// walls; from cell (1, 1), knock down the wall to a random unvisited cell two steps away
// (up, right, down, left, picked by a seeded generator: state = state × 48271 mod 2³¹ − 1)
// and move there; at a dead end, back up. Every cell gets carved exactly once, so the
// maze is a tree: exactly one path between any two cells. Output: the grid, row by row
// (0 open, 1 wall).
import { fail } from "../engine/codecs";
import { DIRS } from "../engine/grid";
import { tokens, type InputError } from "../engine/inputs";
import { Recorder } from "../engine/recorder";
import type { NarrationKey, NarrationParams, Role, Step, TracerDef } from "../engine/types";

type Key = NarrationKey<"maze-generation">;
const KEYS = ["maze-generation.ready", "maze-generation.carve", "maze-generation.back", "maze-generation.done"] as const satisfies readonly Key[];
const say = (key: (typeof KEYS)[number], params?: NarrationParams<number>) => ({ key, params });

export const MARKERS = ["carve", "back"] as const;
export const MIN_K = 2;
export const MAX_K = 7;

function trace(input: readonly number[]): Step[] {
  const [k = MIN_K, seed = 1] = input;
  const size = 2 * k + 1;
  const cells = new Array<number>(size * size).fill(1);
  const at = (r: number, c: number) => r * size + c;
  let state = seed;
  const rand = () => {
    state = (state * 48271) % 2147483647;
    return state;
  };
  const r = new Recorder([]);
  const stack: [number, number][] = [[1, 1]];
  const done = new Set<number>();
  cells[at(1, 1)] = 0;
  const snap = (narration: ReturnType<typeof say>, marker: string | null, extra: Record<number, Role> = {}) => {
    const roles: Record<number, Role> = {};
    cells.forEach((v, j) => {
      if (v === 0) roles[j] = "sorted";
    });
    for (const [sr, sc] of stack) roles[at(sr, sc)] = "pivot";
    for (const j of done) roles[j] = "out";
    if (stack.length) roles[at(...stack[stack.length - 1]!)] = "key";
    Object.assign(roles, extra);
    r.snap({
      narration,
      marker,
      grid: { rows: size, cols: size, cells: [...cells], start: null, goal: null, roles, text: {}, path: [] },
      vars: { depth: stack.length, carved: r.writes + 1 },
    });
  };

  snap(say("maze-generation.ready", { size, cells: k * k, seed }), null);
  while (stack.length) {
    const [cr, cc] = stack[stack.length - 1]!;
    const options: [number, number][] = [];
    for (const [dr, dc] of DIRS) {
      const nr = cr + 2 * dr;
      const nc = cc + 2 * dc;
      if (nr > 0 && nr < size && nc > 0 && nc < size && cells[at(nr, nc)] === 1) options.push([dr, dc]);
    }
    if (options.length) {
      const pick = rand() % options.length;
      const [dr, dc] = options[pick]!;
      cells[at(cr + dr, cc + dc)] = 0;
      cells[at(cr + 2 * dr, cc + 2 * dc)] = 0;
      stack.push([cr + 2 * dr, cc + 2 * dc]);
      r.writes++;
      snap(say("maze-generation.carve", { r: cr + 2 * dr, c: cc + 2 * dc, options: options.length, pick: pick + 1 }), "carve", { [at(cr + dr, cc + dc)]: "swap" });
    } else {
      done.add(at(cr, cc));
      stack.pop();
      r.moves++;
      snap(say("maze-generation.back", { r: cr, c: cc }), "back");
    }
  }
  // Finished: every open cell alike, so the maze reads as a whole.
  done.clear();
  snap(say("maze-generation.done", { size, cells: k * k }), null);
  r.steps[r.steps.length - 1]!.result = [...cells];
  return r.steps;
}

export const mazeGeneration: TracerDef = {
  id: "maze-generation",
  subject: "grid-pathfinding",
  scene: "grid",
  limits: { min_len: 2, max_len: 2, min: 1, max: 9999 },
  input: {
    kind: "codec",
    codec: {
      fields: ["size", "seed"],
      format: (input) => [String(input[0] ?? MIN_K), String(input[1] ?? 1)],
      parse: (texts) => {
        const out: number[] = [];
        for (const [j, field] of (["size", "seed"] as const).entries()) {
          const t = tokens(texts[j] ?? "");
          if ("error" in t) return t;
          if (t.values.length !== 1) return fail("one_number", { field });
          out.push(t.values[0]!);
        }
        return { values: out };
      },
      check: (input): InputError | null => {
        if (input[0]! < MIN_K || input[0]! > MAX_K) return { code: "custom", key: "maze_size", params: { min: MIN_K, max: MAX_K } };
        if (input[1]! < 1 || input[1]! > 9999) return { code: "custom", key: "maze_seed", params: { max: 9999 } };
        return null;
      },
    },
  },
  presets: {
    small: () => [3, 7],
    random: (next) => [MIN_K + Math.floor(next() * (MAX_K - MIN_K + 1)), 1 + Math.floor(next() * 9999)],
    large: () => [MAX_K, 2024],
  },
  markers: MARKERS,
  narrationKeys: KEYS,
  counters: [
    { slot: "writes", label: "carved" },
    { slot: "moves", label: "backtracks" },
  ],
  legend: ["key", "pivot", "swap", "sorted", "out"],
  roleLabels: { key: "current", pivot: "on_the_path", swap: "wall_removed", sorted: "open", out: "dead_end" },
  vars: ["depth", "carved"],
  trace,
};
