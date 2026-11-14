import { describe, expect, it } from "vitest";
import { adjacency, decodeGraph, encodeGraph, graphCodec, randomGraph } from "./graph";
import { layoutGraph } from "./graphLayout";
import { rng } from "./inputs";

const errKey = (r: ReturnType<ReturnType<typeof graphCodec>["parse"]>) => ("error" in r && r.error.code === "custom" ? r.error.key : null);

describe("graph inputs", () => {
  const weighted = graphCodec({ directed: false, weighted: true, wMin: 1, wMax: 9 });
  const plain = graphCodec({ directed: true, weighted: false });
  const choose = graphCodec({ directed: "choose", weighted: true });
  const negative = graphCodec({ directed: true, weighted: true, wMin: -5, wMax: 9 });

  it("parses a node count and an edge list, and formats it back", () => {
    const r = weighted.parse(["4", "0 1 3, 1 2 5; 2 3 1"]);
    expect(r).toEqual({ values: [4, 0, 0, 1, 3, 1, 2, 5, 2, 3, 1] });
    expect(weighted.format([4, 0, 0, 1, 3, 1, 2, 5, 2, 3, 1])).toEqual(["4", "0 1 3, 1 2 5, 2 3 1"]);
    expect(plain.parse(["3", "0 1, 1 2"])).toEqual({ values: [3, 1, 0, 1, 1, 1, 2, 1] });
    expect(plain.format([3, 1, 0, 1, 1, 1, 2, 1])).toEqual(["3", "0 1, 1 2"]);
    expect(choose.parse(["2", "1", "0 1 4"])).toEqual({ values: [2, 1, 0, 1, 4] });
    expect(negative.parse(["2", "0 1 -3"])).toEqual({ values: [2, 1, 0, 1, -3] });
  });

  it("explains every mistake", () => {
    expect(errKey(weighted.parse(["4", "0 1"]))).toBe("edge_weighted");
    expect(errKey(plain.parse(["3", "0 1 2"]))).toBe("edge_pair");
    expect(errKey(plain.parse(["3", "0 0"]))).toBe("self_loop");
    expect(errKey(plain.parse(["3", "0 1, 0 1"]))).toBe("duplicate_edge");
    expect(errKey(weighted.parse(["3", "0 1 2, 1 0 2"]))).toBe("duplicate_edge");
    expect(plain.parse(["3", "0 1, 1 0"])).toHaveProperty("values");
    expect(errKey(plain.parse(["3", "0 3"]))).toBe("node_range");
    expect(errKey(plain.parse(["13", "0 1"]))).toBe("node_count");
    expect(errKey(plain.parse(["3", ""]))).toBe("edge_count");
    expect(errKey(weighted.parse(["3", "0 1 12"]))).toBe("weight_range");
    expect(errKey(choose.parse(["3", "2", "0 1 1"]))).toBe("directed_flag");
    expect(plain.check!([3, 0, 0, 1, 1])).toMatchObject({ key: "directed_flag" });
    const flow = graphCodec({ directed: true, weighted: true, antiparallel: false });
    expect(errKey(flow.parse(["3", "0 1 2, 1 0 3"]))).toBe("antiparallel");
    expect(flow.parse(["3", "0 1 2, 1 2 3"])).toHaveProperty("values");
  });

  it("builds adjacency lists in kb_graph's order", () => {
    const g = decodeGraph(encodeGraph(3, false, [[0, 1, 4], [2, 0, 1]]));
    expect(adjacency(g).map((l) => l.map((x) => [x.v, x.w]))).toEqual([[[1, 4], [2, 1]], [[0, 4]], [[0, 1]]]);
  });

  it("makes random graphs that pass its own check", () => {
    for (let seed = 1; seed <= 40; seed++) {
      const next = rng(seed);
      const g = randomGraph(next, { nMin: 4, nMax: 12, directed: seed % 2 === 0, weighted: true, density: 1.6, connected: true });
      const codec = graphCodec({ directed: seed % 2 === 0, weighted: true });
      expect(codec.check!(g), `seed ${seed}`).toBeNull();
    }
  });

  it("lays a graph out the same way every time, inside the unit square", () => {
    const edges = [
      { u: 0, v: 1 },
      { u: 1, v: 2 },
      { u: 2, v: 0 },
      { u: 2, v: 3 },
    ];
    const a = layoutGraph(4, edges);
    const b = layoutGraph(4, [...edges]);
    expect(b).toBe(a);
    for (const p of a.pos) {
      expect(p.x).toBeGreaterThanOrEqual(0);
      expect(p.x).toBeLessThanOrEqual(1);
      expect(p.y).toBeGreaterThanOrEqual(0);
      expect(p.y).toBeLessThanOrEqual(1);
    }
    const spread = Math.min(...a.pos.flatMap((p, i) => a.pos.slice(i + 1).map((q) => Math.hypot(p.x - q.x, p.y - q.y))));
    expect(spread).toBeGreaterThan(0.1);
  });
});
