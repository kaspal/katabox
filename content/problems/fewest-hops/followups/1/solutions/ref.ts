export function hopsToAll(n: number, links: number[][]): number[] {
  const adj: number[][] = Array.from({ length: n }, () => []);
  for (const [a, b] of links) {
    adj[a].push(b);
    adj[b].push(a);
  }
  const dist = new Array<number>(n).fill(-1);
  dist[0] = 0;
  const q = [0];
  for (let h = 0; h < q.length; h++) {
    const u = q[h];
    for (const v of adj[u])
      if (dist[v] === -1) {
        dist[v] = dist[u] + 1;
        q.push(v);
      }
  }
  return dist;
}
