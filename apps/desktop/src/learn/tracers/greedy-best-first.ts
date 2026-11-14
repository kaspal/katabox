// Greedy best-first search on a grid. Mirrors content/learn/grid-pathfinding/greedy-best-first/sample.{py,ts}: The queue orders cells by h alone, the Manhattan distance to G: fast toward the goal, but the path it finds can cost more than the best one. A cell is never revisited. The label in a cell is its h.
// Output: [the cost of the path found] or [-1]. The shared logic is in gridSearch.ts.
import { encodeMap, gridCodec, randomGrid } from "../engine/grid";
import type { NarrationKey, TracerDef } from "../engine/types";
import { gridSearch } from "./gridSearch";

type Key = NarrationKey<"greedy-best-first">;
const KEYS = [
  "greedy-best-first.ready",
  "greedy-best-first.pop",
  "greedy-best-first.goal",
  "greedy-best-first.discover",
  "greedy-best-first.done",
  "greedy-best-first.no_path",
] as const satisfies readonly Key[];

export const MARKERS = ["pop", "goal", "discover"] as const;

export const greedyBestFirst: TracerDef = {
  id: "greedy-best-first",
  subject: "grid-pathfinding",
  scene: "grid",
  compareGroup: "grid",
  limits: { min_len: 10, max_len: 198, min: 0, max: 16 },
  input: { kind: "codec", codec: gridCodec({ weights: true }) },
  presets: {
    trap: () => encodeMap("......../.#####../.....#../S....#.G/.....#../.#####../........"),
    random: (next) => randomGrid(next, { rows: [6, 9], cols: [8, 12], walls: 0.2, weights: 0.2 }),
    "toll-lane": () => encodeMap("S..9..G/......./......."),
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
  trace: (input) => gridSearch(input, "greedy", "greedy-best-first"),
};
