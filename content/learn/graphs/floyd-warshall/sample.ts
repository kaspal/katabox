const INF = 999;

function floydWarshall(n: number, edges: number[][]): number[] {
  const dist = Array.from({ length: n }, (_, i) => Array.from({ length: n }, (_, j) => (i === j ? 0 : INF)));
  for (const [u, v, w] of edges) dist[u][v] = w;
  for (let k = 0; k < n; k++) {
    const through = dist[k]; // @step pivot
    for (let i = 0; i < n; i++) {
      for (let j = 0; j < n; j++) {
        if (dist[i][k] + through[j] < dist[i][j]) {
          dist[i][j] = dist[i][k] + through[j]; // @step improve
        }
      }
    }
  }
  return dist.flat();
}
