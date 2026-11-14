import { describe, expect, it } from "vitest";
import { comparePartners } from "./compare";
import { decodeGrid, encodeMap, formatMap, gridCodec, randomGrid } from "./grid";
import { rng } from "./inputs";
import { TRACERS } from "../tracers";

const errKey = (r: ReturnType<ReturnType<typeof gridCodec>["parse"]>) => ("error" in r && r.error.code === "custom" ? r.error.key : null);

describe("grid inputs", () => {
  const codec = gridCodec({ weights: true });

  it("reads a map and writes it back", () => {
    const r = codec.parse(["S.#/.3./..g"]);
    expect(r).toEqual({ values: [3, 3, 0, 0, 2, 2, 0, 0, 1, 0, 3, 0, 0, 0, 0] });
    expect(formatMap(decodeGrid(encodeMap("S.#/.3./..G")))).toBe("S.#/.3./..G");
    expect(codec.format([3, 3, 0, 0, 2, 2, 0, 0, 1, 0, 3, 0, 0, 0, 0])).toEqual(["S.#/.3./..G"]);
    expect(codec.parse(["S.\n.G"])).toEqual({ values: [2, 2, 0, 0, 1, 1, 0, 0, 0, 0] });
  });

  it("explains every mistake", () => {
    expect(errKey(codec.parse(["S.G"]))).toBe("grid_size");
    expect(errKey(codec.parse(["S../.G"]))).toBe("grid_ragged");
    expect(errKey(codec.parse(["S../..."]))).toBe("grid_start_goal");
    expect(errKey(codec.parse(["SS./..G"]))).toBe("grid_start_goal");
    expect(errKey(codec.parse(["S.x/..G"]))).toBe("grid_cell");
    expect(errKey(gridCodec({ weights: false }).parse(["S.5/..G"]))).toBe("grid_no_weights");
    expect(errKey(codec.parse([`${"S".padEnd(17, ".")}/${"G".padEnd(17, ".")}`]))).toBe("grid_size");
  });

  it("makes random grids that pass its own check", () => {
    for (let seed = 1; seed <= 40; seed++) {
      const g = randomGrid(rng(seed), { rows: [2, 12], cols: [2, 16], walls: 0.3, weights: 0.3 });
      expect(codec.check!(g), `seed ${seed}`).toBeNull();
    }
  });

  it("races BFS, Dijkstra, A* and greedy on the same map", () => {
    const maze = encodeMap("S.#....../.##.###.#/....#...#/##.##.#.#/...#..#.G");
    const bfs = TRACERS["grid-bfs"]!;
    expect(comparePartners(bfs, Object.values(TRACERS), maze).map((d) => d.id).sort()).toEqual(["a-star", "greedy-best-first", "grid-dijkstra"]);
  });
});
