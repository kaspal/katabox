const MOD = 1_000_003;

export function countRoutes(n: number, links: number[][]): number {
  const adj: number[][] = Array.from({ length: n }, () => []);
  for (const [a, b] of links) {
    adj[a].push(b);
    adj[b].push(a);
  }
  const dist = new Array<number>(n).fill(-1);
  const ways = new Array<number>(n).fill(0);
  dist[0] = 0;
  ways[0] = 1;
  const queue = [0];
  for (let head = 0; head < queue.length; head++) {
    const u = queue[head];
    for (const v of adj[u]) {
      if (dist[v] === -1) {
        dist[v] = dist[u] + 1;
        ways[v] = ways[u];
        queue.push(v);
      } else if (dist[v] === dist[u] + 1) {
        ways[v] = (ways[v] + ways[u]) % MOD;
      }
    }
  }
  return ways[n - 1] % MOD;
}
