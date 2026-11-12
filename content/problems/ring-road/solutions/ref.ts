export function ringLength(n: number, roads: number[][]): number {
  const near: number[][] = Array.from({ length: n }, () => []);
  for (const [a, b] of roads) {
    near[a].push(b);
    near[b].push(a);
  }
  const degree = near.map((x) => x.length);
  const removed = new Uint8Array(n);
  const queue: number[] = [];
  for (let v = 0; v < n; v++) if (degree[v] === 1) queue.push(v);
  let left = n;
  for (let h = 0; h < queue.length; h++) {
    const v = queue[h];
    removed[v] = 1;
    left--;
    for (const u of near[v]) {
      if (!removed[u] && --degree[u] === 1) queue.push(u);
    }
  }
  return left;
}
