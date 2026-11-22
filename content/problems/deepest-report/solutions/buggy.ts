export function deepestReport(manager: number[]): number {
  const n = manager.length;
  const depth = new Array<number>(n).fill(0);
  const level = (v: number): number => {
    if (manager[v] === -1) return 1;
    if (depth[v] === 0) depth[v] = level(manager[v]) + 1;
    return depth[v];
  };
  let best = 0;
  for (let v = 0; v < n; v++) best = Math.max(best, level(v));
  return best;
}
