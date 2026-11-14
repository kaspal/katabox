function zeroOneBfs(n: number, adj: number[][][], source: number): number[] {
  const dist = new Array<number>(n).fill(-1);
  dist[source] = 0;
  const queue = [source];
  while (queue.length > 0) {
    const u = queue.shift()!; // @step pop
    for (const [v, w] of adj[u]) {
      if (dist[v] === -1 || dist[u] + w < dist[v]) {
        dist[v] = dist[u] + w;
        if (w === 0) {
          queue.unshift(v); // @step front
        } else {
          queue.push(v); // @step back
        }
      }
    }
  }
  return dist;
}
