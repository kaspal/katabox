export function stepOrder(n: number, before: number[][]): number[] {
  const waiting = new Int32Array(n);
  const out: number[][] = Array.from({ length: n }, () => []);
  for (const [a, b] of before) {
    out[a].push(b);
    waiting[b]++;
  }
  // A binary min-heap of ready steps.
  const heap: number[] = [];
  const push = (x: number) => {
    heap.push(x);
    let i = heap.length - 1;
    while (i > 0) {
      const p = (i - 1) >> 1;
      if (heap[p] <= heap[i]) break;
      [heap[p], heap[i]] = [heap[i], heap[p]];
      i = p;
    }
  };
  const pop = (): number => {
    const top = heap[0];
    const last = heap.pop()!;
    if (heap.length > 0) {
      heap[0] = last;
      let i = 0;
      for (;;) {
        const l = 2 * i + 1;
        const r = l + 1;
        let m = i;
        if (l < heap.length && heap[l] < heap[m]) m = l;
        if (r < heap.length && heap[r] < heap[m]) m = r;
        if (m === i) break;
        [heap[i], heap[m]] = [heap[m], heap[i]];
        i = m;
      }
    }
    return top;
  };
  for (let s = 0; s < n; s++) if (waiting[s] === 0) push(s);
  const order: number[] = [];
  while (heap.length > 0) {
    const s = pop();
    order.push(s);
    for (const t of out[s]) {
      if (--waiting[t] === 0) push(t);
    }
  }
  return order.length === n ? order : [];
}
