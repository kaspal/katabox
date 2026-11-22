function maxFlow(n: number, edges: number[][]): number[] {
  const cap = Array.from({ length: n }, () => new Array<number>(n).fill(0));
  for (const [u, v, c] of edges) cap[u][v] = c;
  const source = 0;
  const sink = n - 1;
  let total = 0;
  for (;;) {
    const parent = new Array<number>(n).fill(-1);
    parent[source] = source;
    const queue = [source];
    while (queue.length > 0 && parent[sink] === -1) {
      const u = queue.shift()!;
      for (let v = 0; v < n; v++) {
        if (parent[v] === -1 && cap[u][v] > 0) {
          parent[v] = u; // @step reach
          queue.push(v);
        }
      }
    }
    if (parent[sink] === -1) return [total];
    let push = 999;
    for (let v = sink; v !== source; v = parent[v]) {
      push = Math.min(push, cap[parent[v]][v]);
    }
    for (let v = sink; v !== source; v = parent[v]) {
      cap[parent[v]][v] -= push; // @step augment
      cap[v][parent[v]] += push;
    }
    total += push;
  }
}
