export function nearestStalls(stalls: number[][], k: number): number[] {
  // A max-heap of the best k (distance, index) keys.
  const heap: [number, number][] = [];
  const bigger = (a: [number, number], b: [number, number]) => a[0] > b[0] || (a[0] === b[0] && a[1] > b[1]);
  const up = (i: number) => {
    while (i > 0) {
      const p = (i - 1) >> 1;
      if (!bigger(heap[i], heap[p])) break;
      [heap[i], heap[p]] = [heap[p], heap[i]];
      i = p;
    }
  };
  const down = (i: number) => {
    for (;;) {
      const l = 2 * i + 1;
      const r = l + 1;
      let m = i;
      if (l < heap.length && bigger(heap[l], heap[m])) m = l;
      if (r < heap.length && bigger(heap[r], heap[m])) m = r;
      if (m === i) return;
      [heap[i], heap[m]] = [heap[m], heap[i]];
      i = m;
    }
  };
  stalls.forEach(([x, y], i) => {
    const key: [number, number] = [x * x + y * y, i];
    if (heap.length < k) {
      heap.push(key);
      up(heap.length - 1);
    } else if (bigger(heap[0], key)) {
      heap[0] = key;
      down(0);
    }
  });
  return heap.sort((a, b) => a[0] - b[0] || a[1] - b[1]).map((e) => e[1]);
}
