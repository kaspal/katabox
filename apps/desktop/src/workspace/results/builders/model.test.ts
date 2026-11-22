import { describe, expect, it } from "vitest";
import {
  addChild,
  buildersFor,
  decodeTree,
  encodeTree,
  gridCharset,
  nodeCount,
  placeTree,
  removeNode,
  setCell,
} from "./model";

describe("builder model (expansion M9)", () => {
  it("round-trips level-order trees", () => {
    for (const level of [[], [1], [1, 2, 3], [5, 3, 8, null, 4], [1, null, 2, null, 3], [1, 2, null, 3, null, 4]]) {
      expect(encodeTree(decodeTree(level)!)).toEqual(level);
    }
    expect(decodeTree([null, 1])).toBeNull();
    expect(decodeTree("x")).toBeNull();
  });

  it("adds and removes nodes, keeping the encoding canonical", () => {
    let t = decodeTree([])!;
    t = addChild(t, null, "left", 7);
    t = addChild(t, t.root, "right", 9);
    expect(encodeTree(t)).toEqual([7, null, 9]);
    t = addChild(t, t.root, "left", 1);
    expect(encodeTree(t)).toEqual([7, 1, 9]);
    t = removeNode(t, t.right[t.root!]!);
    expect(encodeTree(t)).toEqual([7, 1]);
    expect(placeTree(t).map((p) => [p.x, p.depth])).toEqual([
      [0, 1],
      [1, 0],
    ]);
  });

  it("offers builders by type and name", () => {
    expect(buildersFor("TreeNode", "root", [])).toEqual(["tree"]);
    expect(buildersFor("int[]", "nums", [])).toEqual(["rows"]);
    expect(buildersFor("int", "k", 1)).toEqual([]);
    expect(buildersFor("string[]", "floor", ["..", "#."])[0]).toBe("grid");
    expect(buildersFor("string[]", "words", ["ab", "c"])[0]).toBe("rows");
    expect(buildersFor("int[][]", "roads", [[0, 1, 5]])[0]).toBe("graph");
    expect(buildersFor("int[][]", "matrix", [[0, 1]])[0]).toBe("rows");
  });

  it("grids and graphs", () => {
    expect(gridCharset(["a#", ".b"])).toEqual(["a", "#", ".", "b"]);
    expect(setCell(["..", ".."], 1, 0, "#")).toEqual(["..", "#."]);
    expect(nodeCount([[0, 3], [1, 2]])).toBe(4);
    expect(nodeCount([], 2)).toBe(2);
  });
});
