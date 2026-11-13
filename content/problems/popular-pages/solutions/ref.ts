export function popularPages(capacity: number, ops: string[], args: number[][]): number[] {
  const value = new Map<number, number>();
  const count = new Map<number, number>();
  // count -> pages with that count, in insertion (= last use) order.
  const bucket = new Map<number, Set<number>>();
  let minCount = 0;
  const out: number[] = [];
  const addTo = (c: number, page: number) => {
    let set = bucket.get(c);
    if (set === undefined) {
      set = new Set();
      bucket.set(c, set);
    }
    set.add(page);
  };
  const use = (page: number) => {
    const c = count.get(page)!;
    const set = bucket.get(c)!;
    set.delete(page);
    if (set.size === 0) {
      bucket.delete(c);
      if (minCount === c) minCount = c + 1;
    }
    count.set(page, c + 1);
    addTo(c + 1, page);
  };
  ops.forEach((op, i) => {
    const page = args[i][0];
    if (op === "get") {
      if (value.has(page)) {
        use(page);
        out.push(value.get(page)!);
      } else {
        out.push(-1);
      }
    } else if (value.has(page)) {
      value.set(page, args[i][1]);
      use(page);
    } else {
      if (value.size === capacity) {
        const set = bucket.get(minCount)!;
        const old = set.values().next().value!;
        set.delete(old);
        if (set.size === 0) bucket.delete(minCount);
        value.delete(old);
        count.delete(old);
      }
      value.set(page, args[i][1]);
      count.set(page, 1);
      addTo(1, page);
      minCount = 1;
    }
  });
  return out;
}
