export function clubsLeft(n: number, merges: number[][]): number[] {
  const parent = Int32Array.from({ length: n }, (_, i) => i);
  const size = new Int32Array(n).fill(1);
  const find = (x: number): number => {
    while (parent[x] !== x) {
      parent[x] = parent[parent[x]];
      x = parent[x];
    }
    return x;
  };
  let clubs = n;
  return merges.map(([a, b]) => {
    let ra = find(a);
    let rb = find(b);
    if (ra !== rb) {
      if (size[ra] < size[rb]) [ra, rb] = [rb, ra];
      parent[rb] = ra;
      size[ra] += size[rb];
      clubs--;
    }
    return clubs;
  });
}
