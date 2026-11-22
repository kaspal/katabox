export function favouritePages(capacity: number, ops: string[], args: number[][]): number[] {
  const value = new Map<number, number>();
  const freq = new Map<number, number>();
  const buckets = new Map<number, Set<number>>(); // Sets keep insertion order
  let least = 0;
  const out: number[] = [];
  const touch = (page: number) => {
    const f = freq.get(page)!;
    const b = buckets.get(f)!;
    b.delete(page);
    if (b.size === 0) {
      buckets.delete(f);
      if (least === f) least = f + 1;
    }
    freq.set(page, f + 1);
    let nb = buckets.get(f + 1);
    if (!nb) buckets.set(f + 1, (nb = new Set()));
    nb.add(page);
  };
  for (let i = 0; i < ops.length; i++) {
    const a = args[i];
    if (ops[i] === "get") {
      const v = value.get(a[0]);
      if (v === undefined) out.push(-1);
      else {
        touch(a[0]);
        out.push(v);
      }
      continue;
    }
    const [page, v] = a;
    if (value.has(page)) {
      value.set(page, v);
      touch(page);
      continue;
    }
    if (value.size === capacity) {
      const b = buckets.get(least)!;
      const old = b.values().next().value as number;
      b.delete(old);
      if (b.size === 0) buckets.delete(least);
      value.delete(old);
      freq.delete(old);
    }
    value.set(page, v);
    freq.set(page, 1);
    let one = buckets.get(1);
    if (!one) buckets.set(1, (one = new Set()));
    one.add(page);
    least = 1;
  }
  return out;
}
