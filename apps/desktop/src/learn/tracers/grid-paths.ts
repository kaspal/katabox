// Paths through a grid with blocked cells, moving only right or down. Mirrors
// content/learn/dynamic-programming/grid-paths/sample.{py,ts}: a path reaches a cell from
// above or from the left, so ways = ways above + ways to the left; a blocked cell has 0
// ways, and the top-left cell has 1 (unless it is blocked). The table has the grid's
// shape. Output: [the ways to reach the bottom-right cell].
import { blocksCodec } from "../engine/shapes";
import { Recorder } from "../engine/recorder";
import type { NarrationKey, NarrationParams, Step, TracerDef } from "../engine/types";

type Key = NarrationKey<"grid-paths">;
const KEYS = ["grid-paths.ready", "grid-paths.blocked", "grid-paths.start", "grid-paths.add", "grid-paths.done"] as const satisfies readonly Key[];
const say = (key: (typeof KEYS)[number], params?: NarrationParams<number>) => ({ key, params });

export const MARKERS = ["blocked", "add"] as const;
export const enc = (map: string) => {
  const lines = map.split("/");
  return [lines.length, lines[0]!.length, ...[...lines.join("")].map((ch) => (ch === "#" ? 1 : 0))];
};

function trace(input: readonly number[]): Step[] {
  const [rows = 1, cols = 1] = input;
  const cells = input.slice(2);
  const r = new Recorder([]);
  const ways: (number | null)[][] = Array.from({ length: rows }, () => new Array(cols).fill(null));
  const blocked: [number, number][] = [];
  cells.forEach((v, k) => {
    if (v === 1) blocked.push([Math.floor(k / cols), k % cols]);
  });
  const snap = (narration: ReturnType<typeof say>, marker: string | null, hot: [number, number] | null = null, deps: [number, number][] = []) =>
    r.snap({
      narration,
      marker,
      dp: { label: "paths", colLabels: Array.from({ length: cols }, (_, c) => String(c)), rowLabels: Array.from({ length: rows }, (_, k) => String(k)), cells: ways.map((row) => [...row]), hot, deps, blocked },
      vars: { r: hot ? hot[0] : null, c: hot ? hot[1] : null },
    });

  snap(say("grid-paths.ready", { rows, cols, blocked: blocked.length }), null);
  for (let rr = 0; rr < rows; rr++) {
    for (let c = 0; c < cols; c++) {
      if (cells[rr * cols + c] === 1) {
        ways[rr]![c] = 0;
        snap(say("grid-paths.blocked", { r: rr, c }), "blocked", [rr, c]);
      } else if (rr === 0 && c === 0) {
        ways[rr]![c] = 1;
        snap(say("grid-paths.start"), null, [rr, c]);
      } else {
        const up = rr > 0 ? ways[rr - 1]![c]! : 0;
        const left = c > 0 ? ways[rr]![c - 1]! : 0;
        ways[rr]![c] = up + left;
        r.writes++;
        const deps: [number, number][] = [];
        if (rr > 0) deps.push([rr - 1, c]);
        if (c > 0) deps.push([rr, c - 1]);
        snap(say("grid-paths.add", { r: rr, c, up, left, ways: up + left }), "add", [rr, c], deps);
      }
    }
  }
  snap(say("grid-paths.done", { ways: ways[rows - 1]![cols - 1]! }), null);
  r.steps[r.steps.length - 1]!.result = [ways[rows - 1]![cols - 1]!];
  return r.steps;
}

export const gridPaths: TracerDef = {
  id: "grid-paths",
  subject: "dynamic-programming",
  scene: "dp",
  limits: { min_len: 4, max_len: 66, min: 0, max: 8 },
  input: { kind: "codec", codec: blocksCodec({ rows: 8, cols: 8 }) },
  presets: {
    "a-few-blocks": () => enc("....../.#..#./....../..#.../......"),
    random: (next) => {
      const rows = 2 + Math.floor(next() * 6);
      const cols = 2 + Math.floor(next() * 6);
      const cells = Array.from({ length: rows * cols }, (_, k) => (k > 0 && k < rows * cols - 1 && next() < 0.2 ? 1 : 0));
      return [rows, cols, ...cells];
    },
    "walled-off": () => enc("..../.##./.#../#..."),
  },
  markers: MARKERS,
  narrationKeys: KEYS,
  counters: [{ slot: "writes", label: "writes" }],
  legend: ["key", "compare", "sorted"],
  roleLabels: { key: "current", compare: "read", sorted: "filled" },
  vars: ["r", "c"],
  trace,
};
