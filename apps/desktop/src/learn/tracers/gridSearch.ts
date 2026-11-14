// Shared tracer for the grid searches (ADR 0029, Phase 6): BFS, Dijkstra, A* and greedy
// best-first differ only in what orders their frontier and when a cell counts as settled.
// Each mirrors its own content/learn/grid-pathfinding/<id>/sample.{py,ts}. Neighbours are tried up,
// right, down, left. Output: [the cost of the path found] or [-1].
import { DIRS, decodeGrid, gridData, type Grid } from "../engine/grid";
import { Recorder } from "../engine/recorder";
import type { Role, Step } from "../engine/types";

export type Kind = "bfs" | "dijkstra" | "astar" | "greedy";
type Say = (key: string, params?: Record<string, number>) => { key: never; params?: Record<string, number> };

export const cost = (v: number) => (v >= 2 ? v : 1);

export function gridSearch(input: readonly number[], kind: Kind, id: string): Step[] {
  const say: Say = (key, params) => ({ key: `${id}.${key}` as never, params });
  const g: Grid = decodeGrid(input);
  const { rows, cols } = g;
  const at = (r: number, c: number) => r * cols + c;
  const goal = at(...g.goal);
  const start = at(...g.start);
  const h = (k: number) => Math.abs(Math.floor(k / cols) - g.goal[0]) + Math.abs((k % cols) - g.goal[1]);
  const r = new Recorder([]);
  // dist: BFS steps, Dijkstra / A* g, greedy "paid so far".
  const dist = new Map<number, number>([[start, 0]]);
  const parent = new Map<number, number>();
  const explored = new Set<number>();
  // Frontier entries, kept in the order the sample's queue would hand them out.
  type Entry = { key: number[]; k: number };
  let frontier: Entry[] = [{ key: kind === "astar" ? [h(start), h(start), ...g.start] : kind === "greedy" ? [h(start), ...g.start] : [0, ...g.start], k: start }];
  const ordered = kind !== "bfs";
  const sortFrontier = () => {
    frontier = [...frontier].sort((a, b) => {
      for (let j = 0; j < a.key.length; j++) if (a.key[j] !== b.key[j]) return a.key[j]! - b.key[j]!;
      return 0;
    });
  };
  const pathTo = (k: number) => {
    const path = [k];
    while (path[path.length - 1] !== start) path.push(parent.get(path[path.length - 1]!)!);
    return path.reverse();
  };
  const label = (k: number) => {
    const d = dist.get(k)!;
    return kind === "astar" ? `${d + h(k)}` : kind === "greedy" ? `h${h(k)}` : `${d}`;
  };
  const snap = (narration: ReturnType<Say>, marker: string | null, cur: number | null, extra: Record<number, Role> = {}, path: number[] = []) => {
    const roles: Record<number, Role> = {};
    for (const k of explored) roles[k] = "sorted";
    for (const e of frontier) if (!explored.has(e.k) || kind !== "bfs") roles[e.k] = roles[e.k] === "sorted" ? "sorted" : "pivot";
    if (cur !== null) roles[cur] = "key";
    Object.assign(roles, extra);
    const text: Record<number, string> = {};
    for (const k of dist.keys()) if (k !== start && k !== goal) text[k] = label(k);
    r.snap({
      narration,
      marker,
      grid: gridData(g, { roles, text, path }),
      vars: { explored: explored.size, frontier: frontier.length, cost: cur === null ? null : dist.get(cur)! },
    });
  };

  snap(say("ready", { rows, cols }), null, null);
  while (frontier.length) {
    const e = frontier[0]!;
    frontier = frontier.slice(1);
    const k = e.k;
    const [cr, cc] = [Math.floor(k / cols), k % cols];
    if (ordered && kind !== "greedy") {
      // Stale: an older, longer entry for a cell already settled cheaper.
      const d = kind === "astar" ? e.key[0]! - e.key[1]! : e.key[0]!;
      snap(say("pop", { r: cr, c: cc, d }), "pop", k);
      if (d > dist.get(k)!) {
        snap(say("stale", { r: cr, c: cc, d, best: dist.get(k)! }), "stale", k, { [k]: "out" });
        continue;
      }
    } else {
      snap(say(kind === "bfs" ? "visit" : "pop", { r: cr, c: cc, d: dist.get(k)!, h: h(k) }), kind === "bfs" ? "visit" : "pop", k);
    }
    explored.add(k);
    r.comparisons++;
    if (k === goal) {
      const path = pathTo(k);
      snap(say("goal", { d: dist.get(k)!, explored: explored.size, len: path.length - 1 }), "goal", k, Object.fromEntries(path.map((p) => [p, "swap" as Role])), path);
      snap(say("done", { d: dist.get(k)!, explored: explored.size }), null, null, Object.fromEntries(path.map((p) => [p, "swap" as Role])), path);
      r.steps[r.steps.length - 1]!.result = [dist.get(k)!];
      return r.steps;
    }
    for (const [dr, dc] of DIRS) {
      const nr = cr + dr;
      const nc = cc + dc;
      if (nr < 0 || nr >= rows || nc < 0 || nc >= cols || g.cells[at(nr, nc)] === 1) continue;
      const n = at(nr, nc);
      const step = kind === "bfs" ? 1 : cost(g.cells[n]!);
      const nd = dist.get(k)! + step;
      const better = kind === "bfs" || kind === "greedy" ? !dist.has(n) : !dist.has(n) || nd < dist.get(n)!;
      if (!better) continue;
      dist.set(n, nd);
      parent.set(n, k);
      r.writes++;
      if (kind === "bfs") frontier.push({ key: [], k: n });
      else if (kind === "dijkstra") frontier.push({ key: [nd, nr, nc], k: n });
      else if (kind === "astar") frontier.push({ key: [nd + h(n), h(n), nr, nc], k: n });
      else frontier.push({ key: [h(n), nr, nc], k: n });
      if (ordered) sortFrontier();
      const marker = kind === "bfs" || kind === "greedy" ? "discover" : "relax";
      snap(say(marker, { r: nr, c: nc, d: nd, h: h(n), f: nd + h(n) }), marker, k, { [n]: "compare" });
    }
  }
  snap(say("no_path", { explored: explored.size }), null, null);
  r.steps[r.steps.length - 1]!.result = [-1];
  return r.steps;
}
