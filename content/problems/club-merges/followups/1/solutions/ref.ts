export function biggestClub(n: number, merges: number[][]): number[] {
  const parent = Array.from({ length: n }, (_, i) => i);
  const size = new Array<number>(n).fill(1);
  const find = (x: number) => {
    while (parent[x] !== x) {
      parent[x] = parent[parent[x]];
      x = parent[x];
    }
    return x;
  };
  let biggest = 1;
  return merges.map(([a, b]) => {
    let ra = find(a);
    let rb = find(b);
    if (ra !== rb) {
      if (size[ra] < size[rb]) [ra, rb] = [rb, ra];
      parent[rb] = ra;
      size[ra] += size[rb];
      biggest = Math.max(biggest, size[ra]);
    }
    return biggest;
  });
}
