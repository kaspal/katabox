function topoOrder(n: number, adj: number[][][]): number[] {
  const indegree = new Array<number>(n).fill(0);
  for (let u = 0; u < n; u++) {
    for (const [v] of adj[u]) indegree[v]++;
  }
  const queue: number[] = [];
  for (let u = 0; u < n; u++) if (indegree[u] === 0) queue.push(u);
  const order: number[] = [];
  let head = 0;
  while (head < queue.length) {
    const u = queue[head++];
    order.push(u); // @step take
    for (const [v] of adj[u]) {
      indegree[v]--; // @step lower
      if (indegree[v] === 0) {
        queue.push(v); // @step ready
      }
    }
  }
  return order;
}
