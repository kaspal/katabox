function dfs(n: number, adj: number[][][], source: number): number[] {
  const seen = new Array<boolean>(n).fill(false);
  const order: number[] = [];
  const finished: number[] = [];

  function visit(u: number): void {
    seen[u] = true;
    order.push(u); // @step enter
    for (const [v] of adj[u]) {
      if (!seen[v]) {
        visit(v); // @step go
      }
    }
    finished.push(u); // @step leave
  }

  visit(source);
  return [...order, ...finished];
}
