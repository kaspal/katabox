// Convex hull, Andrew's monotone chain. Mirrors content/learn/math-geometry/convex-hull/sample.{py,ts}.
import { int } from "../engine/inputs";
import { Recorder } from "../engine/recorder";
import type { NarrationKey, Role, Step, TracerDef } from "../engine/types";
import { MAX_COORD, cross, pointsCodec, pts, type P } from "./geometry";

type HullKey = NarrationKey<"convex-hull">;
const HULL_KEYS = ["convex-hull.ready", "convex-hull.sorted", "convex-hull.drop", "convex-hull.keep", "convex-hull.upper", "convex-hull.done"] as const satisfies readonly HullKey[];

function hullTrace(input: readonly number[]): Step[] {
  const all = pts(input);
  const unique = [...new Map(all.map((p) => [`${p[0]},${p[1]}`, p])).values()].sort((a, b) => a[0] - b[0] || a[1] - b[1]);
  const r = new Recorder([]);
  const say = (key: (typeof HULL_KEYS)[number], params?: Record<string, number>) => ({ key, params });
  let chain: P[] = [];
  let lowerDone: P[] = [];
  const snap = (narration: ReturnType<typeof say>, marker: string | null, o: { turn?: [P, P, P]; turnRole?: Role; current?: P; closed?: boolean } = {}) => {
    const onChain = new Set([...lowerDone, ...chain].map((p) => `${p[0]},${p[1]}`));
    r.snap({
      narration,
      marker,
      plane: {
        max: MAX_COORD,
        points: unique.map((p, id) => ({ id, x: p[0], y: p[1], label: `${id}`, role: o.current && o.current[0] === p[0] && o.current[1] === p[1] ? "key" : onChain.has(`${p[0]},${p[1]}`) ? "sorted" : undefined })),
        segments: [],
        polygon: [...lowerDone, ...chain],
        closed: o.closed,
        turn: o.turn,
        turnRole: o.turnRole,
      },
      vars: { chain: chain.length },
    });
  };
  snap(say("convex-hull.ready", { n: all.length }), null);
  if (unique.length <= 2) {
    chain = unique;
    snap(say("convex-hull.done", { n: unique.length }), null, { closed: true });
    r.steps[r.steps.length - 1]!.result = unique.flat();
    return r.steps;
  }
  snap(say("convex-hull.sorted", { n: unique.length }), null);
  const pass = (order: P[], drop: string, keep: string) => {
    chain = [];
    for (const p of order) {
      while (chain.length >= 2 && cross(chain[chain.length - 2]!, chain[chain.length - 1]!, p) <= 0) {
        r.comparisons++;
        const turn: [P, P, P] = [chain[chain.length - 2]!, chain[chain.length - 1]!, p];
        const gone = chain.pop()!;
        r.moves++;
        snap(say("convex-hull.drop", { x: gone[0], y: gone[1] }), drop, { turn, turnRole: "swap", current: p });
      }
      if (chain.length >= 2) r.comparisons++;
      chain.push(p);
      snap(say("convex-hull.keep", { x: p[0], y: p[1] }), keep, { current: p });
    }
  };
  pass(unique, "drop", "keep");
  const lower = chain;
  lowerDone = lower.slice(0, -1);
  snap(say("convex-hull.upper"), null);
  pass([...unique].reverse(), "drop-upper", "keep-upper");
  const hull = [...lower.slice(0, -1), ...chain.slice(0, -1)];
  lowerDone = [];
  chain = hull;
  snap(say("convex-hull.done", { n: hull.length }), null, { closed: true });
  r.steps[r.steps.length - 1]!.result = hull.flat();
  return r.steps;
}

export const convexHull: TracerDef = {
  id: "convex-hull",
  subject: "math-geometry",
  scene: "plane",
  limits: { min_len: 2, max_len: 24, min: 0, max: MAX_COORD },
  input: { kind: "codec", codec: pointsCodec({ min: 1, max: 12 }) },
  presets: {
    "a-few-inside": () => [2, 3, 9, 1, 15, 4, 17, 11, 11, 16, 4, 13, 8, 7, 12, 9, 6, 10],
    random: (next) => Array.from({ length: 2 * int(next, 5, 12) }, () => int(next, 0, MAX_COORD)),
    square: () => [4, 4, 16, 4, 16, 16, 4, 16, 10, 10, 10, 4, 4, 10],
  },
  markers: ["drop", "keep", "drop-upper", "keep-upper"],
  narrationKeys: HULL_KEYS,
  counters: [
    { slot: "comparisons", label: "turns_checked" },
    { slot: "moves", label: "dropped" },
  ],
  legend: ["key", "sorted", "swap"],
  roleLabels: { key: "current", sorted: "on_the_hull", swap: "clockwise_turn" },
  vars: ["chain"],
  trace: hullTrace,
};
