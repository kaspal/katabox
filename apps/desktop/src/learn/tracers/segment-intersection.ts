// Segment intersection by orientation tests. Mirrors
// content/learn/math-geometry/segment-intersection/sample.{py,ts}.
import { int } from "../engine/inputs";
import { Recorder } from "../engine/recorder";
import type { NarrationKey, NarrationParams, Role, Step, TracerDef } from "../engine/types";
import { MAX_COORD, cross, pointsCodec, pts, type P } from "./geometry";

type SegKey = NarrationKey<"segment-intersection">;
const SEG_KEYS = ["segment-intersection.ready", "segment-intersection.side_left", "segment-intersection.side_right", "segment-intersection.side_on", "segment-intersection.cross", "segment-intersection.touch", "segment-intersection.apart"] as const satisfies readonly SegKey[];
const NAMES = ["P1", "P2", "P3", "P4"];

function segTrace(input: readonly number[]): Step[] {
  const [p1, p2, p3, p4] = pts(input) as [P, P, P, P];
  const r = new Recorder([]);
  const say = (key: (typeof SEG_KEYS)[number], params?: NarrationParams) => ({ key, params: params as NarrationParams<number> });
  const checks: [P, P, P, string][] = [
    [p3, p4, p1, "P1"],
    [p3, p4, p2, "P2"],
    [p1, p2, p3, "P3"],
    [p1, p2, p4, "P4"],
  ];
  const signs: number[] = [];
  const snap = (narration: ReturnType<typeof say>, marker: string | null, o: { k?: number; result?: Role } = {}) => {
    const segRole = (seg: 0 | 1): Role | undefined => (o.result ? o.result : o.k === undefined ? undefined : (o.k < 2 ? 1 : 0) === seg ? "compare" : undefined);
    r.snap({
      narration,
      marker,
      plane: {
        max: MAX_COORD,
        points: [p1, p2, p3, p4].map((p, id) => ({ id, x: p[0], y: p[1], label: `${NAMES[id]} ${signs[id] === undefined ? "" : signs[id]! > 0 ? "+" : signs[id]! < 0 ? "−" : "0"}`.trim(), role: o.k === id ? "key" : undefined })),
        segments: [
          { a: p1, b: p2, role: segRole(0) },
          { a: p3, b: p4, role: segRole(1) },
        ],
        turn: o.k === undefined ? undefined : [checks[o.k]![0], checks[o.k]![1], checks[o.k]![2]],
      },
      vars: { d1: signs[0] ?? null, d2: signs[1] ?? null, d3: signs[2] ?? null, d4: signs[3] ?? null },
    });
  };
  snap(say("segment-intersection.ready"), null);
  checks.forEach(([a, b, c, name], k) => {
    signs.push(Math.sign(cross(a, b, c)) || 0);
    r.comparisons++;
    const key = signs[k]! > 0 ? "segment-intersection.side_left" : signs[k]! < 0 ? "segment-intersection.side_right" : "segment-intersection.side_on";
    snap(say(key, { point: name, segment: k < 2 ? "P3P4" : "P1P2" }), "side", { k });
  });
  const onSeg = (a: P, b: P, c: P) => Math.min(a[0], b[0]) <= c[0] && c[0] <= Math.max(a[0], b[0]) && Math.min(a[1], b[1]) <= c[1] && c[1] <= Math.max(a[1], b[1]);
  if (signs[0]! * signs[1]! < 0 && signs[2]! * signs[3]! < 0) {
    snap(say("segment-intersection.cross"), "cross", { result: "sorted" });
    r.steps[r.steps.length - 1]!.result = [1, ...signs];
    return r.steps;
  }
  for (let k = 0; k < 4; k++) {
    const [a, b, c, name] = checks[k]!;
    if (signs[k] === 0 && onSeg(a, b, c)) {
      snap(say("segment-intersection.touch", { point: name }), "touch", { result: "sorted" });
      r.steps[r.steps.length - 1]!.result = [1, ...signs];
      return r.steps;
    }
  }
  snap(say("segment-intersection.apart"), null, { result: "out" });
  r.steps[r.steps.length - 1]!.result = [0, ...signs];
  return r.steps;
}

export const segmentIntersection: TracerDef = {
  id: "segment-intersection",
  subject: "math-geometry",
  scene: "plane",
  limits: { min_len: 8, max_len: 8, min: 0, max: MAX_COORD },
  input: { kind: "codec", codec: pointsCodec({ min: 4, max: 4, field: "segments" }) },
  presets: {
    crossing: () => [2, 3, 15, 14, 3, 16, 17, 2],
    random: (next) => Array.from({ length: 8 }, () => int(next, 0, MAX_COORD)),
    "end-on-the-line": () => [3, 3, 15, 9, 9, 6, 13, 16],
    "side-by-side": () => [2, 2, 8, 14, 10, 2, 16, 14],
  },
  markers: ["side", "cross", "touch"],
  narrationKeys: SEG_KEYS,
  counters: [{ slot: "comparisons", label: "orientation_tests" }],
  legend: ["key", "compare", "sorted", "out"],
  roleLabels: { key: "current", compare: "tested_against", sorted: "they_meet", out: "apart" },
  vars: ["d1", "d2", "d3", "d4"],
  trace: segTrace,
};
