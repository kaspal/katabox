export function ferryTimes(n: number, routes: number[][], queries: number[][]): number[] {
  const dist: number[][] = Array.from({ length: n }, (_, i) => Array.from({ length: n }, (_, j) => (i === j ? 0 : Infinity)));
  for (const [a, b, w] of routes) {
    if (w < dist[a][b]) {
      dist[a][b] = w;
      dist[b][a] = w;
    }
  }
  for (let k = 0; k < n; k++) {
    for (let i = 0; i < n; i++) {
      const dik = dist[i][k];
      if (dik === Infinity) continue;
      for (let j = 0; j < n; j++) {
        if (dik + dist[k][j] < dist[i][j]) dist[i][j] = dik + dist[k][j];
      }
    }
  }
  return queries.map(([a, b]) => (dist[a][b] === Infinity ? -1 : dist[a][b]));
}
