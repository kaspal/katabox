// Graph layout for the Learn scenes (ADR 0029, Phase 4): d3-force, settled once. Nodes
// start on a circle, the simulation runs a fixed number of ticks with a seeded random
// source, and the result is cached per graph shape, so positions are the same on every
// machine and never move during playback. Only the editor's "Apply" (a new graph)
// lays out again.
import { forceCollide, forceLink, forceManyBody, forceSimulation, forceX, forceY, type SimulationNodeDatum } from "d3-force";

type P = SimulationNodeDatum & { id: number };
/** Positions in [0, 1] × [0, 1], and the settled drawing's height / width. */
export type GraphLayout = { pos: { x: number; y: number }[]; aspect: number };

const cache = new Map<string, GraphLayout>();
const TICKS = 300;

export function layoutGraph(n: number, edges: readonly { u: number; v: number }[]): GraphLayout {
  const key = `${n}|${edges.map((e) => `${Math.min(e.u, e.v)}-${Math.max(e.u, e.v)}`).join(",")}`;
  const hit = cache.get(key);
  if (hit) return hit;
  // Node 0 at the top, the rest clockwise.
  const nodes: P[] = Array.from({ length: n }, (_, id) => ({
    id,
    x: 100 * Math.sin((2 * Math.PI * id) / Math.max(1, n)),
    y: -100 * Math.cos((2 * Math.PI * id) / Math.max(1, n)),
  }));
  const links = edges.filter((e) => e.u !== e.v).map((e) => ({ source: e.u, target: e.v }));
  let seed = 7;
  const random = () => {
    seed = (seed * 1664525 + 1013904223) % 4294967296;
    return seed / 4294967296;
  };
  const sim = forceSimulation<P>(nodes)
    .randomSource(random)
    .force("link", forceLink<P, { source: number; target: number }>(links).id((d) => d.id).distance(70).strength(0.7))
    .force("charge", forceManyBody<P>().strength(-260))
    .force("collide", forceCollide<P>(30))
    .force("x", forceX<P>(0).strength(0.06))
    .force("y", forceY<P>(0).strength(0.1))
    .stop();
  for (let k = 0; k < TICKS; k++) sim.tick();
  const xs = nodes.map((p) => p.x!);
  const ys = nodes.map((p) => p.y!);
  const [x0, x1, y0, y1] = [Math.min(...xs), Math.max(...xs), Math.min(...ys), Math.max(...ys)];
  const pos = nodes.map((p) => ({
    x: x1 - x0 < 1e-6 ? 0.5 : (p.x! - x0) / (x1 - x0),
    y: y1 - y0 < 1e-6 ? 0.5 : (p.y! - y0) / (y1 - y0),
  }));
  const out = { pos, aspect: x1 - x0 < 1e-6 ? 1 : (y1 - y0) / (x1 - x0) };
  cache.set(key, out);
  return out;
}
