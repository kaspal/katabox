class Heap {
  private a: number[] = [];
  private less: (x: number, y: number) => boolean;
  constructor(less: (x: number, y: number) => boolean) {
    this.less = less;
  }
  get size(): number {
    return this.a.length;
  }
  top(): number {
    return this.a[0];
  }
  push(x: number): void {
    const a = this.a;
    a.push(x);
    let i = a.length - 1;
    while (i > 0) {
      const p = (i - 1) >> 1;
      if (!this.less(a[i], a[p])) break;
      [a[i], a[p]] = [a[p], a[i]];
      i = p;
    }
  }
  pop(): number {
    const a = this.a;
    const top = a[0];
    const last = a.pop()!;
    if (a.length > 0) {
      a[0] = last;
      let i = 0;
      for (;;) {
        const l = 2 * i + 1;
        const r = l + 1;
        let m = i;
        if (l < a.length && this.less(a[l], a[m])) m = l;
        if (r < a.length && this.less(a[r], a[m])) m = r;
        if (m === i) break;
        [a[i], a[m]] = [a[m], a[i]];
        i = m;
      }
    }
    return top;
  }
}

export function runningMedians(readings: number[]): number[] {
  const low = new Heap((x, y) => x > y); // max-heap
  const high = new Heap((x, y) => x < y); // min-heap
  const out: number[] = [];
  for (const x of readings) {
    if (low.size === 0 || x <= low.top()) low.push(x);
    else high.push(x);
    if (low.size > high.size + 1) high.push(low.pop());
    else if (high.size > low.size) low.push(high.pop());
    out.push(low.size > high.size ? low.top() : (low.top() + high.top()) / 2);
  }
  return out;
}
