export function fewestHops(n: number, links: number[][]): number {
  const near: number[][] = Array.from({ length: n }, () => []);
  for (const [a, b] of links) {
    near[a].push(b);
    near[b].push(a);
  }
  const dist = new Int32Array(n).fill(-1);
  dist[0] = 0;
  const queue = [0];
  for (let h = 0; h < queue.length; h++) {
    const v = queue[h];
    for (const u of near[v]) {
      if (dist[u] === -1) {
        dist[u] = dist[v] + 1;
        queue.push(u);
      }
    }
  }
  return dist[n - 1];
}
