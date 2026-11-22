// TypeScript has no built-in priority queue: this one keeps (weight, node,
// from) triples sorted, smallest first, which is enough for a dozen nodes.
function prim(n: number, adj: number[][][]): number[] {
  const inTree = new Array<boolean>(n).fill(false);
  const order: number[] = [];
  let total = 0;
  const queue: number[][] = [[0, 0, 0]];
  while (queue.length > 0) {
    const [w, v] = queue.shift()!; // @step pop
    if (inTree[v]) {
      continue; // @step skip
    }
    inTree[v] = true;
    order.push(v);
    total += w; // @step take
    for (const [x, wx] of adj[v]) {
      if (!inTree[x]) {
        queue.push([wx, x, v]); // @step push
        queue.sort((p, q) => p[0] - q[0] || p[1] - q[1] || p[2] - q[2]);
      }
    }
  }
  return [total, ...order];
}
