export function cheapestNetwork(n: number, cables: number[][]): number {
  const parent = new Int32Array(n);
  for (let i = 0; i < n; i++) parent[i] = i;
  const find = (x: number): number => {
    while (parent[x] !== x) {
      parent[x] = parent[parent[x]];
      x = parent[x];
    }
    return x;
  };
  const sorted = [...cables].sort((p, q) => p[2] - q[2]);
  let total = 0;
  let laid = 0;
  for (const [a, b, cost] of sorted) {
    if (laid === n - 1) break;
    const ra = find(a);
    const rb = find(b);
    if (ra !== rb) {
      parent[ra] = rb;
      total += cost;
      laid++;
    }
  }
  return laid === n - 1 ? total : -1;
}
