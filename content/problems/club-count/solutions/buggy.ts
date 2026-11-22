export function clubCount(n: number, friendships: number[][]): number[] {
  const parent = Array.from({ length: n }, (_, i) => i);
  const find = (x: number): number => {
    while (parent[x] !== x) x = parent[x];
    return x;
  };
  let clubs = n;
  const out: number[] = [];
  for (const [a, b] of friendships) {
    if (find(a) !== find(b)) {
      parent[a] = b;
      clubs--;
    }
    out.push(clubs);
  }
  return out;
}
