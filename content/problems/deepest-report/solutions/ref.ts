export function deepestReport(manager: number[]): number {
  const n = manager.length;
  const depth = new Array<number>(n).fill(0);
  let best = 0;
  for (let v = 0; v < n; v++) {
    const path: number[] = [];
    let u = v;
    while (u !== -1 && depth[u] === 0) {
      path.push(u);
      u = manager[u];
    }
    let d = u === -1 ? 0 : depth[u];
    for (let i = path.length - 1; i >= 0; i--) depth[path[i]] = ++d;
    best = Math.max(best, depth[v]);
  }
  return best;
}
