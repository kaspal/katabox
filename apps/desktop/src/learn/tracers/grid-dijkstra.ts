// Dijkstra on a weighted grid. Mirrors content/learn/grid-pathfinding/grid-dijkstra/sample.{py,ts}: Entering a cell costs its weight (an open cell costs 1); a priority queue of (distance, row, column) settles cells cheapest first. The label in a cell is its best distance so far.
// Output: [the cost of the path found] or [-1]. The shared logic is in gridSearch.ts.
import { encodeMap, gridCodec, randomGrid } from "../engine/grid";
import type { NarrationKey, TracerDef } from "../engine/types";
import { gridSearch } from "./gridSearch";

type Key = NarrationKey<"grid-dijkstra">;
const KEYS = [
  "grid-dijkstra.ready",
  "grid-dijkstra.pop",
  "grid-dijkstra.stale",
  "grid-dijkstra.goal",
  "grid-dijkstra.relax",
  "grid-dijkstra.done",
  "grid-dijkstra.no_path",
] as const satisfies readonly Key[];

export const MARKERS = ["pop", "stale", "goal", "relax"] as const;

export const gridDijkstra: TracerDef = {
  id: "grid-dijkstra",
  subject: "grid-pathfinding",
  scene: "grid",
  compareGroup: "grid",
  limits: { min_len: 10, max_len: 198, min: 0, max: 16 },
  input: { kind: "codec", codec: gridCodec({ weights: true }) },
  presets: {
    swamp: () => encodeMap("S..99..../.#.99.##./.#.99..#./...99...G"),
    random: (next) => randomGrid(next, { rows: [6, 9], cols: [8, 12], walls: 0.18, weights: 0.25 }),
    maze: () => encodeMap("S.#....../.##.###.#/....#...#/##.##.#.#/...#..#.G"),
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
  trace: (input) => gridSearch(input, "dijkstra", "grid-dijkstra"),
};
