// A* search on a grid. Mirrors content/learn/grid-pathfinding/a-star/sample.{py,ts}: Like Dijkstra, but the queue orders cells by f = g + h, where h is the Manhattan distance to G (it never overestimates, so the answer stays optimal); ties go to the smaller h. The label in a cell is its f.
// Output: [the cost of the path found] or [-1]. The shared logic is in gridSearch.ts.
import { encodeMap, gridCodec, randomGrid } from "../engine/grid";
import type { NarrationKey, TracerDef } from "../engine/types";
import { gridSearch } from "./gridSearch";

type Key = NarrationKey<"a-star">;
const KEYS = [
  "a-star.ready",
  "a-star.pop",
  "a-star.stale",
  "a-star.goal",
  "a-star.relax",
  "a-star.done",
  "a-star.no_path",
] as const satisfies readonly Key[];

export const MARKERS = ["pop", "stale", "goal", "relax"] as const;

export const aStar: TracerDef = {
  id: "a-star",
  subject: "grid-pathfinding",
  scene: "grid",
  compareGroup: "grid",
  limits: { min_len: 10, max_len: 198, min: 0, max: 16 },
  input: { kind: "codec", codec: gridCodec({ weights: true }) },
  presets: {
    maze: () => encodeMap("S.#....../.##.###.#/....#...#/##.##.#.#/...#..#.G"),
    random: (next) => randomGrid(next, { rows: [6, 9], cols: [8, 12], walls: 0.18, weights: 0.2 }),
    swamp: () => encodeMap("S..99..../.#.99.##./.#.99..#./...99...G"),
  },
  markers: MARKERS,
  narrationKeys: KEYS,
  counters: [
    { slot: "comparisons", label: "explored" },
    { slot: "writes", label: "relaxed" },
  ],
  legend: ["key", "pivot", "compare", "sorted", "swap", "out"],
  roleLabels: { key: "current", pivot: "in_queue", compare: "relaxed", sorted: "explored", swap: "path", out: "stale" },
  vars: ["explored", "frontier", "cost"],
  trace: (input) => gridSearch(input, "astar", "a-star"),
};
