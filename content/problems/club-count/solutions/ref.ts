export function clubCount(n: number, friendships: number[][]): number[] {
  const parent = Array.from({ length: n }, (_, i) => i);
  const find = (x: number): number => {
    while (parent[x] !== x) {
      parent[x] = parent[parent[x]];
      x = parent[x];
    }
    return x;
  };
  let clubs = n;
  const out: number[] = [];
  for (const [a, b] of friendships) {
    const ra = find(a);
    const rb = find(b);
    if (ra !== rb) {
      parent[ra] = rb;
      clubs--;
    }
    out.push(clubs);
  }
  return out;
}
