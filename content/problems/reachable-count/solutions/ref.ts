export function reachableCount(n: number, doors: number[][], start: number): number {
  const adj: number[][] = Array.from({ length: n }, () => []);
  for (const [a, b] of doors) {
    adj[a].push(b);
    adj[b].push(a);
  }
  const seen = new Array<boolean>(n).fill(false);
  seen[start] = true;
  const queue = [start];
  for (let head = 0; head < queue.length; head++) {
    for (const u of adj[queue[head]]) {
      if (!seen[u]) {
        seen[u] = true;
        queue.push(u);
      }
    }
  }
  return queue.length;
}
