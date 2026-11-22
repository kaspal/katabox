function kruskal(n: number, edges: number[][]): number[] {
  const parent = Array.from({ length: n }, (_, k) => k);
  const find = (x: number): number => {
    while (parent[x] !== x) {
      parent[x] = parent[parent[x]];
      x = parent[x];
    }
    return x;
  };
  let total = 0;
  const used: number[] = [];
  const order = [...edges].sort(
    (a, b) => a[2] - b[2] || a[0] - b[0] || a[1] - b[1],
  );
  for (const [u, v, w] of order) {
    const ru = find(u);
    const rv = find(v); // @step consider
    if (ru === rv) {
      continue; // @step skip
    }
    parent[ru] = rv;
    total += w; // @step join
    used.push(u, v);
  }
  return [total, ...used];
}
