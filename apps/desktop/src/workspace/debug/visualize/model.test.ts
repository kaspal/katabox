import { describe, expect, it } from "vitest";
import type { DebugSnapshot, SnapObject, SnapValue, VarInfo } from "@/bindings";
import { diffModels, hasChanges, nodeIndex } from "@/workspace/debug/visualize/diff";
import { layoutGraph, layoutTree } from "@/workspace/debug/visualize/layout";
import {
  DEFAULT_SETTINGS,
  MAX_GRAPH_NODES,
  chosenNames,
  isPointer,
  modelFor,
  modelsOf,
  parseSettings,
  shownOf,
  type GraphModel,
  type Model,
  type TreeModel,
  type VizSettings,
} from "@/workspace/debug/visualize/model";

const int = (n: number): SnapValue => ({ t: "int", v: String(n) });
const none: SnapValue = { t: "none", v: "None" };
const ref = (id: number): SnapValue => ({ t: "ref", id });
const local = (name: string, value: number): VarInfo => ({ name, type: "int", value: String(value), children: 0 });

/** A snapshot from objects, with roots by name. */
function snapshot(objects: SnapObject[], roots: Record<string, SnapValue>): DebugSnapshot {
  return { objects, roots: Object.entries(roots).map(([name, value]) => ({ name, value })), truncated: false, error: null };
}

const list = (items: SnapValue[], len = items.length): SnapObject => ({ kind: "list", type: "list", items, len, typed: false });

/** A linked list of `vals` (objects 0..n-1), with optional input numbers and a cycle. */
function linked(vals: number[], opts: { inputs?: (number | null)[]; cycleTo?: number } = {}): SnapObject[] {
  return vals.map((v, i) => ({
    kind: "list_node",
    val: int(v),
    next: i + 1 < vals.length ? ref(i + 1) : opts.cycleTo !== undefined ? ref(opts.cycleTo) : none,
    input: opts.inputs ? opts.inputs[i] : i,
  }));
}

/** A complete binary tree of n nodes (objects 0..n-1, children of i at 2i+1, 2i+2). */
function tree(n: number): SnapObject[] {
  return Array.from({ length: n }, (_, i) => ({
    kind: "tree_node" as const,
    val: int(i),
    left: 2 * i + 1 < n ? ref(2 * i + 1) : none,
    right: 2 * i + 2 < n ? ref(2 * i + 2) : none,
    input: i,
  }));
}

function model(snap: DebugSnapshot, name: string, locals: VarInfo[] = [], settings: VizSettings = DEFAULT_SETTINGS): Model {
  const root = snap.roots!.find((r) => r.name === name)!;
  return modelFor(name, root.value, snap, locals, settings);
}

describe("Visualizer model (ADR 0027)", () => {
  it("detects each shape", () => {
    const s = snapshot(
      [
        list([int(3), int(5), int(8)]),
        list([ref(2), ref(3)]),
        list([int(1), int(2)]),
        list([int(3), int(4)]),
        { kind: "map", type: "dict", entries: [[{ t: "str", v: "a" }, int(1)]], len: 1 },
        { kind: "set", type: "set", items: [int(7)], len: 1 },
        ...linked([1, 2]).map((o) => ({ ...o, next: o.kind === "list_node" && o.next.t === "ref" ? ref(o.next.id + 6) : none }) as SnapObject),
      ],
      { arr: ref(0), grid: ref(1), map: ref(4), set: ref(5), head: ref(6), x: int(4) },
    );
    expect(model(s, "arr").kind).toBe("array");
    expect(model(s, "grid")).toMatchObject({ kind: "grid", len: 2, cols: 2 });
    expect(model(s, "map")).toMatchObject({ kind: "map", entries: [{ key: "a", value: "1" }] });
    expect(model(s, "set").kind).toBe("set");
    expect(model(s, "head")).toMatchObject({ kind: "linked", cycleTo: null, more: false });
    expect(model(s, "x")).toEqual({ kind: "scalar", name: "x", text: "4" });
    // A heap only when asked for.
    const heap = model(s, "arr", [], { ...DEFAULT_SETTINGS, as: { arr: "heap" } }) as TreeModel;
    expect(heap).toMatchObject({ kind: "tree", heap: true });
    expect(heap.nodes[0]).toMatchObject({ text: "3", left: 1, right: 2 });
  });

  it("shows pointers into arrays, with the user's choice overriding the default both ways", () => {
    const s = snapshot([list([int(1), int(2), int(3)])], { nums: ref(0) });
    const locals = [local("i", 0), local("hi", 3), local("count", 2), local("j", 9)];
    const m = model(s, "nums", locals);
    // i (default) at 0, hi (default) at the end (3 = len); count is not a default; j is out of range.
    expect(m.kind === "array" && m.pointers).toEqual([
      { name: "i", index: 0 },
      { name: "hi", index: 3 },
    ]);
    const settings: VizSettings = { ...DEFAULT_SETTINGS, pointers: { i: false, count: true } };
    expect(isPointer("i", settings)).toBe(false);
    expect(isPointer("count", settings)).toBe(true);
    const custom = model(s, "nums", locals, settings);
    expect(custom.kind === "array" && custom.pointers.map((p) => p.name)).toEqual(["hi", "count"]);
  });

  it("draws a cycle once and labels the nodes other variables point at", () => {
    const objects = linked([3, 8, 1], { cycleTo: 1 });
    const s = snapshot(objects, { head: ref(0), slow: ref(1), fast: ref(1) });
    const m = model(s, "head");
    expect(m.kind).toBe("linked");
    if (m.kind !== "linked") return;
    expect(m.nodes.map((n) => n.text)).toEqual(["3", "8", "1"]);
    expect(m.cycleTo).toBe(1);
    expect(m.nodes[1].labels).toEqual(["slow", "fast"]);
    expect(m.nodes[2].next).toBe("i1");
  });

  it("builds graphs only when asked, merging duplicate and two-way edges and keeping self-loops", () => {
    // Adjacency list: 0 → 1 twice, 1 → 0 (two-way), 2 → 2 (self-loop).
    const s = snapshot(
      [list([ref(1), ref(2), ref(3)]), list([int(1), int(1)]), list([int(0)]), list([int(2)])],
      { adj: ref(0) },
    );
    expect(model(s, "adj").kind).toBe("grid");
    const g = model(s, "adj", [], { ...DEFAULT_SETTINGS, as: { adj: "graph" } }) as GraphModel;
    expect(g.kind).toBe("graph");
    expect(g.nodes).toEqual(["0", "1", "2"]);
    expect(g.edges).toEqual([
      { a: 0, b: 1, count: 2, both: true },
      { a: 2, b: 2, count: 1, both: false },
    ]);
    // A 0/1 matrix.
    const m = snapshot([list([ref(1), ref(2)]), list([int(0), int(1)]), list([int(0), int(0)])], { m: ref(0) });
    const mg = model(m, "m", [], { ...DEFAULT_SETTINGS, as: { m: "graph" } }) as GraphModel;
    expect(mg.edges).toEqual([{ a: 0, b: 1, count: 1, both: false }]);
  });

  it("labels weighted edges from [neighbour, weight] pairs, once for an undirected edge", () => {
    // adj[0] = [[1, 5], [2, 7]], adj[1] = [[0, 5]], adj[2] = [[0, 9]] (directions differ).
    const s = snapshot(
      [
        list([ref(1), ref(2), ref(3)]),
        list([ref(4), ref(5)]),
        list([ref(6)]),
        list([ref(7)]),
        list([int(1), int(5)]),
        list([int(2), int(7)]),
        list([int(0), int(5)]),
        list([int(0), int(9)]),
      ],
      { adj: ref(0) },
    );
    const g = model(s, "adj", [], { ...DEFAULT_SETTINGS, as: { adj: "graph" } }) as GraphModel;
    expect(g.edges).toEqual([
      { a: 0, b: 1, count: 1, both: true, weight: "5" },
      { a: 0, b: 2, count: 1, both: true, weight: "7 / 9" },
    ]);
    // A dict of lists of pairs: {"a": [("b", 3)]}.
    const str = (v: string): SnapValue => ({ t: "str", v });
    const d = snapshot(
      [{ kind: "map", type: "dict", entries: [[str("a"), ref(1)]], len: 1 }, list([ref(2)]), list([str("b"), int(3)])],
      { g: ref(0) },
    );
    const dg = model(d, "g", [], { ...DEFAULT_SETTINGS, as: { g: "graph" } }) as GraphModel;
    expect(dg.nodes.length).toBe(2);
    expect(dg.edges).toEqual([{ a: 0, b: 1, count: 1, both: false, weight: "3" }]);
  });

  it("draws a typed number array (array.array, Int32Array) as an array", () => {
    const typed: SnapObject = { kind: "list", type: "Int32Array", items: [int(3), int(1), int(2)], len: 3, typed: true };
    const m = model(snapshot([typed], { nums: ref(0) }), "nums");
    expect(m.kind).toBe("array");
  });

  it(`keeps at most ${MAX_GRAPH_NODES} graph nodes and says how many there are`, () => {
    const n = 80;
    const rows = Array.from({ length: n }, (_, i) => list([int((i + 1) % n)]));
    const s = snapshot([list(rows.map((_, i) => ref(i + 1))), ...rows], { adj: ref(0) });
    const g = model(s, "adj", [], { ...DEFAULT_SETTINGS, as: { adj: "graph" } }) as GraphModel;
    expect(g.nodes.length).toBe(MAX_GRAPH_NODES);
    expect(shownOf(g)).toEqual({ shown: MAX_GRAPH_NODES, total: n });
    expect(g.edges.every((e) => e.a < MAX_GRAPH_NODES && e.b < MAX_GRAPH_NODES)).toBe(true);
    const { points } = layoutGraph(g);
    expect(new Set(points.map((p) => `${p.x.toFixed(1)},${p.y.toFixed(1)}`)).size).toBe(MAX_GRAPH_NODES);
  });

  it("lays out a 31-node tree in order, without overlaps", () => {
    const s = snapshot(tree(31), { root: ref(0) });
    const m = model(s, "root") as TreeModel;
    expect(m.nodes.length).toBe(31);
    const { points, width } = layoutTree(m);
    const spots = new Set(points.map((p) => `${p.x},${p.y}`));
    expect(spots.size).toBe(31);
    // In-order: every left child is left of its parent, every right child right of it.
    m.nodes.forEach((n, i) => {
      if (n.left !== null) expect(points[n.left].x).toBeLessThan(points[i].x);
      if (n.right !== null) expect(points[n.right].x).toBeGreaterThan(points[i].x);
      if (n.left !== null) expect(points[n.left].y).toBeGreaterThan(points[i].y);
    });
    expect(width).toBeLessThan(31 * 50);
  });

  it("reports what it shows fewer of", () => {
    const s = snapshot([list([int(1), int(2)], 100_000)], { big: ref(0) });
    expect(shownOf(model(s, "big"))).toEqual({ shown: 2, total: 100_000 });
    // A list cut by the snapshot's caps ends in an opaque value.
    const cut = snapshot([{ kind: "list_node", val: int(1), next: { t: "opaque", v: "ListNode" }, input: null }], { h: ref(0) });
    expect(shownOf(model(cut, "h"))).toEqual({ shown: 1, total: null });
  });

  it("reads and defaults its per-problem settings", () => {
    expect(parseSettings(null)).toBeUndefined();
    expect(parseSettings({ show: ["a", 1], as: { a: "graph", b: "nope" }, pointers: { i: false, j: "x" } })).toEqual({
      show: ["a"],
      as: { a: "graph" },
      pointers: { i: false },
    });
    // The larger view stays open until the user closes it.
    expect(parseSettings({ show: null, as: {}, pointers: {}, large: true })?.large).toBe(true);
    expect(parseSettings({ show: null, as: {}, pointers: {}, large: "yes" })?.large).toBeUndefined();
    expect(chosenNames(DEFAULT_SETTINGS, ["nums", "target"])).toEqual(["nums", "target"]);
    expect(chosenNames({ ...DEFAULT_SETTINGS, show: ["seen"] }, ["nums"])).toEqual(["seen"]);
  });
});

describe("Visualizer changes since the previous step (ADR 0027)", () => {
  it("marks changed cells and moved pointers", () => {
    const before = model(snapshot([list([int(1), int(2), int(3)])], { a: ref(0) }), "a", [local("i", 0)]);
    const after = model(snapshot([list([int(1), int(9), int(3)])], { a: ref(0) }), "a", [local("i", 1)]);
    const c = diffModels(before, after);
    expect([...c.cells]).toEqual(["1"]);
    expect([...c.pointers]).toEqual(["i"]);
    expect(hasChanges(diffModels(after, after))).toBe(false);
    // The first stop has nothing to compare with.
    expect(hasChanges(diffModels(undefined, after))).toBe(false);
  });

  it("marks a relinked node by its input number, even when the list's order changed", () => {
    // 1 → 2 → 3, then node 0 (input 0) is unlinked and the head moves to node 1.
    const before = model(snapshot(linked([1, 2, 3]), { head: ref(0), cur: ref(0) }), "head");
    const objects = linked([1, 2, 3]);
    objects[0] = { ...objects[0], next: none } as SnapObject;
    const after = model(snapshot(objects, { head: ref(1), cur: ref(1) }), "head");
    const c = diffModels(before, after);
    // Nodes i1 and i2 keep their value and link; cur moved from i0 to i1.
    expect([...c.nodes]).toEqual([]);
    expect([...c.pointers]).toEqual(["cur"]);
    // Relinking node 1 to node 0 (a reversal step) marks node 1.
    const rev = linked([1, 2, 3]);
    rev[1] = { ...rev[1], next: ref(0) } as SnapObject;
    rev[0] = { ...rev[0], next: none } as SnapObject;
    const reversed = model(snapshot(rev, { head: ref(1) }), "head");
    expect([...diffModels(after, reversed).nodes]).toContain("i1");
  });

  it("compares input nodes across drawings: a node moved from `back` to `prev` is marked", () => {
    // Before: back = 6 → 9 (inputs 3, 4), prev = None. After one reversal step: prev = 6
    // (now ending the list), back = 9.
    const node = (v: number, next: SnapValue, input: number): SnapObject => ({ kind: "list_node", val: int(v), next, input });
    const before = snapshot([node(6, ref(1), 3), node(9, none, 4)], { back: ref(0), prev: none });
    const after = snapshot([node(6, none, 3), node(9, none, 4)], { prev: ref(0), back: ref(1) });
    const prevModels = modelsOf(before, ["back", "prev"], [], DEFAULT_SETTINGS);
    const known = nodeIndex(prevModels);
    const [prevCard, backCard] = modelsOf(after, ["prev", "back"], [], DEFAULT_SETTINGS);
    // prev was not a list a step ago, yet node 6 (input 3) is compared: its link changed.
    expect([...diffModels(prevModels[1], prevCard, known).nodes]).toEqual(["i3"]);
    // 9 kept its value and link.
    expect([...diffModels(prevModels[0], backCard, known).nodes]).toEqual([]);
  });

  it("marks tree nodes whose children changed", () => {
    const t = tree(3);
    const before = model(snapshot(t, { root: ref(0) }), "root");
    const swapped = tree(3).map((o, i) => (i === 0 ? { ...o, left: ref(2), right: ref(1) } : o)) as SnapObject[];
    const after = model(snapshot(swapped, { root: ref(0) }), "root");
    expect([...diffModels(before, after).nodes]).toEqual(["i0"]);
  });

  it("marks map keys added or changed, and lists removed ones", () => {
    const map = (entries: [string, number][]): SnapObject => ({
      kind: "map",
      type: "dict",
      entries: entries.map(([k, v]) => [{ t: "str", v: k }, int(v)]),
      len: entries.length,
    });
    const before = model(snapshot([map([["a", 1], ["b", 2]])], { m: ref(0) }), "m");
    const after = model(snapshot([map([["a", 5], ["c", 3]])], { m: ref(0) }), "m");
    const c = diffModels(before, after);
    expect([...c.cells].sort()).toEqual(["a", "c"]);
    expect(c.removed).toEqual(["b"]);
  });
});
