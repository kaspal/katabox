export function longDays(miles: number[], trips: number[][]): number[] {
  const n = miles.length;
  const tree = new Array<number>(n + 1).fill(0);
  const add = (i: number) => {
    for (i += 1; i <= n; i += i & -i) tree[i] += 1;
  };
  const prefix = (i: number) => {
    let s = 0;
    for (; i > 0; i -= i & -i) s += tree[i];
    return s;
  };
  const days = miles.map((_, d) => d).sort((x, y) => miles[y] - miles[x]);
  const order = trips.map((_, j) => j).sort((x, y) => trips[y][2] - trips[x][2]);
  const out = new Array<number>(trips.length).fill(0);
  let p = 0;
  for (const j of order) {
    const [a, b, x] = trips[j];
    while (p < n && miles[days[p]] >= x) {
      add(days[p]);
      p++;
    }
    out[j] = prefix(b + 1) - prefix(a);
  }
  return out;
}
