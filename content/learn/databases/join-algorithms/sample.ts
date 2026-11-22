function hashJoin(a: number[]): number[] {
  const n = a[0];
  const left: [number, number][] = [];
  for (let i = 0; i < n; i++) left.push([a[1 + 2 * i], a[2 + 2 * i]]);
  const rest = a.slice(1 + 2 * n);
  const right: [number, number][] = [];
  for (let i = 0; i + 1 < rest.length; i += 2) {
    right.push([rest[i], rest[i + 1]]);
  }
  const index = new Map<number, number[]>();
  for (const [key, b] of right) {
    index.set(key, [...(index.get(key) ?? []), b]); // @step build
  }
  const out: number[] = [];
  for (const [key, x] of left) {
    const matches = index.get(key) ?? []; // @step probe
    for (const b of matches) {
      out.push(key, x, b); // @step match
    }
  }
  return out;
}
