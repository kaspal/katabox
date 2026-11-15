function groupBy(a: number[]): number[] {
  const groups = new Map<number, [number, number]>();
  for (let i = 0; i + 1 < a.length; i += 2) {
    const [g, v] = [a[i], a[i + 1]];
    const [count, total] = groups.get(g) ?? [0, 0];
    groups.set(g, [count + 1, total + v]); // @step add
  }
  const out: number[] = [];
  for (const g of [...groups.keys()].sort((x, y) => x - y)) {
    out.push(g, ...groups.get(g)!); // @step emit
  }
  return out;
}
