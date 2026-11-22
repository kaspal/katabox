export function hopsOneWay(n: number, links: number[][]): number {
  const fwd: number[][] = Array.from({ length: n }, () => []);
  const back: number[][] = Array.from({ length: n }, () => []);
  for (const [a, b] of links) {
    fwd[a].push(b);
    back[b].push(a);
  }
  const dist = new Array<number>(2 * n).fill(-1); // index station * 2 + flipped
  dist[0] = 0;
  const q = [0];
  for (let h = 0; h < q.length; h++) {
    const s = q[h];
    const u = s >> 1;
    const f = s & 1;
    const d = dist[s];
    for (const v of fwd[u])
      if (dist[v * 2 + f] === -1) {
        dist[v * 2 + f] = d + 1;
        q.push(v * 2 + f);
      }
    if (f === 0)
      for (const v of back[u])
        if (dist[v * 2 + 1] === -1) {
          dist[v * 2 + 1] = d + 1;
          q.push(v * 2 + 1);
        }
  }
  const a = dist[(n - 1) * 2];
  const b = dist[(n - 1) * 2 + 1];
  if (a === -1) return b;
  if (b === -1) return a;
  return Math.min(a, b);
}
