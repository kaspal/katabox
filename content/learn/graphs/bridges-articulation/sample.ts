function bridgesAndCuts(n: number, adj: number[][][]): number[] {
  const tin = new Array<number>(n).fill(-1);
  const low = new Array<number>(n).fill(0);
  const bridges: number[][] = [];
  const cuts = new Set<number>();
  let timer = 0;
  const visit = (u: number, parent: number): void => {
    tin[u] = low[u] = timer; // @step enter
    timer++;
    let children = 0;
    for (const [v] of adj[u]) {
      if (v === parent) continue;
      if (tin[v] !== -1) {
        low[u] = Math.min(low[u], tin[v]); // @step back
      } else {
        visit(v, u);
        children++;
        low[u] = Math.min(low[u], low[v]); // @step low
        if (low[v] > tin[u]) {
          bridges.push([Math.min(u, v), Math.max(u, v)]); // @step bridge
        }
        if (parent !== -1 && low[v] >= tin[u]) {
          cuts.add(u); // @step cut
        }
      }
    }
    if (parent === -1 && children > 1) {
      cuts.add(u); // @step root-cut
    }
  };
  for (let u = 0; u < n; u++) if (tin[u] === -1) visit(u, -1);
  bridges.sort((a, b) => a[0] - b[0] || a[1] - b[1]);
  return [...bridges.flat(), -1, ...[...cuts].sort((a, b) => a - b)];
}
