import { describe, expect, it } from "vitest";
import { blocksCodec, intervalsCodec, itemsCodec, matrixCodec, textWordsCodec } from "./shapes";

type Codec = ReturnType<typeof intervalsCodec>;
const errKey = (r: ReturnType<Codec["parse"]>) => ("error" in r && r.error.code === "custom" ? r.error.key : null);

describe("phase 8 inputs", () => {
  it("intervals", () => {
    const c = intervalsCodec({ min: 0, max: 24, count: [1, 10] });
    expect(c.parse(["1 4, 2 6"])).toEqual({ values: [1, 4, 2, 6] });
    expect(c.format([1, 4, 2, 6])).toEqual(["1 4, 2 6"]);
    expect(errKey(c.parse(["1 4, 2"]))).toBe("interval_pair");
    expect(errKey(c.parse(["5 5"]))).toBe("interval_range");
    expect(errKey(c.parse(["3 30"]))).toBe("interval_range");
    expect(errKey(c.parse([""]))).toBe("interval_count");
  });

  it("items", () => {
    const c = itemsCodec({ capacity: 12, count: 6, weight: 9 });
    expect(c.parse(["10", "5 10, 4 7"])).toEqual({ values: [10, 5, 10, 4, 7] });
    expect(c.format([10, 5, 10, 4, 7])).toEqual(["10", "5 10, 4 7"]);
    expect(errKey(c.parse(["13", "5 10"]))).toBe("capacity_range");
    expect(errKey(c.parse(["10", "5"]))).toBe("item_pair");
    expect(errKey(c.parse(["10", "12 3"]))).toBe("weight_range");
    expect(errKey(c.parse(["10", ""]))).toBe("item_count");
  });

  it("blocked maps", () => {
    const c = blocksCodec({ rows: 8, cols: 8 });
    expect(c.parse([".#/.."])).toEqual({ values: [2, 2, 0, 1, 0, 0] });
    expect(c.format([2, 2, 0, 1, 0, 0])).toEqual([".#/.."]);
    expect(errKey(c.parse([".#/."]))).toBe("grid_ragged");
    expect(errKey(c.parse([".x/.."]))).toBe("grid_no_weights");
    expect(errKey(c.parse(["........./........."]))).toBe("grid_size");
  });

  it("distance matrices", () => {
    const c = matrixCodec({ min: 2, max: 5, weight: 20 });
    expect(c.parse(["0 3, 3 0"])).toEqual({ values: [2, 0, 3, 3, 0] });
    expect(c.format([2, 0, 3, 3, 0])).toEqual(["0 3, 3 0"]);
    expect(errKey(c.parse(["0 3, 4 0"]))).toBe("matrix_symmetric");
    expect(errKey(c.parse(["1 3, 3 0"]))).toBe("matrix_diagonal");
    expect(errKey(c.parse(["0 3 1, 3 0"]))).toBe("matrix_shape");
    expect(errKey(c.parse(["0 30, 30 0"]))).toBe("weight_range");
  });

  it("a text and its words", () => {
    const c = textWordsCodec({ maxText: 12, maxWords: 5, maxWord: 6 });
    expect(c.parse(["abc", "ab c"])).toEqual({ values: [0, 1, 2, -1, 0, 1, -1, 2] });
    expect(c.format([0, 1, 2, -1, 0, 1, -1, 2])).toEqual(["abc", "ab c"]);
    expect(errKey(c.parse(["ab1", "a"]))).toBe("letters_only");
    expect(errKey(c.parse(["abc", ""]))).toBe("word_count");
  });
});
