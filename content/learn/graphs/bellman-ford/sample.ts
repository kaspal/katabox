const INF = 999;

function bellmanFord(n: number, edges: number[][], source: number): number[] {
  const dist = new Array<number>(n).fill(INF);
  dist[source] = 0;
  for (let round = 0; round < n - 1; round++) {
    let changed = false;
    for (const [u, v, w] of edges) {
      if (dist[u] !== INF && dist[u] + w < dist[v]) {
        dist[v] = dist[u] + w; // @step relax
        changed = true;
      }
    }
    if (!changed) {
      break; // @step settled
    }
  }
  for (const [u, v, w] of edges) {
    if (dist[u] !== INF && dist[u] + w < dist[v]) {
      return [1]; // @step negative-cycle
    }
  }
  return [0, ...dist];
}
