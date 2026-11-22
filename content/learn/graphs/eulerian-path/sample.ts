function eulerPath(n: number, edges: number[][]): number[] {
  const adj: number[][][] = Array.from({ length: n }, () => []);
  edges.forEach(([u, v], k) => {
    adj[u].push([v, k]);
    adj[v].push([u, k]);
  });
  const odd = adj
    .map((list, u) => (list.length % 2 === 1 ? u : -1))
    .filter((u) => u !== -1);
  if (odd.length !== 0 && odd.length !== 2) return [-1];
  const start = odd.length > 0 ? odd[0] : edges[0][0];
  const used = new Array<boolean>(edges.length).fill(false);
  const next = new Array<number>(n).fill(0);
  const stack = [start];
  const path: number[] = [];
  while (stack.length > 0) {
    const u = stack[stack.length - 1];
    while (next[u] < adj[u].length && used[adj[u][next[u]][1]]) next[u]++;
    if (next[u] < adj[u].length) {
      const [v, k] = adj[u][next[u]];
      used[k] = true;
      stack.push(v); // @step walk
    } else {
      path.push(stack.pop()!); // @step back
    }
  }
  if (path.length !== edges.length + 1) return [-1];
  return path.reverse();
}
