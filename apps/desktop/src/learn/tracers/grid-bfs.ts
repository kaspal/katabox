// Breadth-first search on a grid. Mirrors content/learn/grid-pathfinding/grid-bfs/sample.{py,ts}: Each step costs 1 (weights are ignored); a FIFO queue explores cells in rings of equal distance. The label in a cell is its distance from S.
// Output: [the cost of the path found] or [-1]. The shared logic is in gridSearch.ts.
import { encodeMap, gridCodec, randomGrid } from "../engine/grid";
import type { NarrationKey, TracerDef } from "../engine/types";
import { gridSearch } from "./gridSearch";

type Key = NarrationKey<"grid-bfs">;
const KEYS = [
  "grid-bfs.ready",
  "grid-bfs.visit",
  "grid-bfs.goal",
  "grid-bfs.discover",
  "grid-bfs.done",
  "grid-bfs.no_path",
] as const satisfies readonly Key[];

export const MARKERS = ["visit", "goal", "discover"] as const;

export const gridBfs: TracerDef = {
  id: "grid-bfs",
  subject: "grid-pathfinding",
  scene: "grid",
  compareGroup: "grid",
  limits: { min_len: 10, max_len: 198, min: 0, max: 16 },
  input: { kind: "codec", codec: gridCodec({ weights: true }) },
  presets: {
    maze: () => encodeMap("S.#....../.##.###.#/....#...#/##.##.#.#/...#..#.G"),
    random: (next) => randomGrid(next, { rows: [6, 9], cols: [8, 12], walls: 0.25 }),
    open: () => encodeMap("S......./......../......../.......G"),
  },
  markers: MARKERS,
  narrationKeys: KEYS,
  counters: [
    { slot: "comparisons", label: "explored" },
    { slot: "writes", label: "discovered" },
  ],
  legend: ["key", "pivot", "compare", "sorted", "swap"],
  roleLabels: { key: "current", pivot: "in_queue", compare: "discovered", sorted: "explored", swap: "path" },
  vars: ["explored", "frontier", "cost"],
  trace: (input) => gridSearch(input, "bfs", "grid-bfs"),
};
