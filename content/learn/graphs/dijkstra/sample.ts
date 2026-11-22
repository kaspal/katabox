// TypeScript has no built-in priority queue: this one keeps (distance, node)
// pairs sorted, smallest first, which is enough for a dozen nodes.
function dijkstra(n: number, adj: number[][][], source: number): number[] {
  const dist: (number | null)[] = new Array(n).fill(null);
  dist[source] = 0;
  const queue: number[][] = [[0, source]];
  while (queue.length > 0) {
    const [d, u] = queue.shift()!; // @step pop
    if (d > dist[u]!) {
      continue; // @step stale
    }
    for (const [v, w] of adj[u]) {
      if (dist[v] === null || d + w < dist[v]!) {
        dist[v] = d + w; // @step relax
        queue.push([d + w, v]);
        queue.sort((x, y) => x[0] - y[0] || x[1] - y[1]);
      }
    }
  }
  return dist.map((x) => (x === null ? -1 : x));
}
