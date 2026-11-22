/** A small binary heap; `before(a, b)` is true when a must come out first. */
class Heap {
  private items: number[] = [];
  private readonly before: (a: number, b: number) => boolean;
  constructor(before: (a: number, b: number) => boolean) {
    this.before = before;
  }
  get size(): number {
    return this.items.length;
  }
  push(x: number): void {
    const a = this.items;
    a.push(x);
    let i = a.length - 1;
    while (i > 0) {
      const p = (i - 1) >> 1;
      if (!this.before(a[i], a[p])) break;
      [a[i], a[p]] = [a[p], a[i]];
      i = p;
    }
  }
  pop(): number {
    const a = this.items;
    const top = a[0];
    const last = a.pop() as number;
    if (a.length > 0) {
      a[0] = last;
      let i = 0;
      for (;;) {
        const l = 2 * i + 1;
        const r = l + 1;
        let m = i;
        if (l < a.length && this.before(a[l], a[m])) m = l;
        if (r < a.length && this.before(a[r], a[m])) m = r;
        if (m === i) break;
        [a[i], a[m]] = [a[m], a[i]];
        i = m;
      }
    }
    return top;
  }
  values(): number[] {
    return [...this.items];
  }
}

export function cheapestK(prices: number[], k: number): number {
  const heap = new Heap((a, b) => a < b);
  for (const p of prices) {
    heap.push(p);
    if (heap.size > k) heap.pop();
  }
  return heap.values().reduce((s, x) => s + x, 0);
}
