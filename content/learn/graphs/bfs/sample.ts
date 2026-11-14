function bfs(n: number, adj: number[][][], source: number): number[] {
  const dist = new Array<number>(n).fill(-1);
  dist[source] = 0;
  const queue = [source];
  let head = 0;
  while (head < queue.length) {
    const u = queue[head++]; // @step visit
    for (const [v] of adj[u]) {
      if (dist[v] === -1) {
        dist[v] = dist[u] + 1; // @step discover
        queue.push(v);
      }
    }
  }
  return dist;
}
