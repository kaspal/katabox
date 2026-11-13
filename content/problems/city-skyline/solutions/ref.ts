export function skyline(buildings: number[][]): number[][] {
  // A max-heap of [height, right], ordered by height.
  const heap: number[][] = [];
  const push = (v: number[]) => {
    heap.push(v);
    let i = heap.length - 1;
    while (i > 0 && heap[(i - 1) >> 1][0] < heap[i][0]) {
      [heap[i], heap[(i - 1) >> 1]] = [heap[(i - 1) >> 1], heap[i]];
      i = (i - 1) >> 1;
    }
  };
  const pop = () => {
    const last = heap.pop()!;
    if (heap.length === 0) return;
    heap[0] = last;
    let i = 0;
    for (;;) {
      const l = 2 * i + 1;
      const r = l + 1;
      let m = i;
      if (l < heap.length && heap[l][0] > heap[m][0]) m = l;
      if (r < heap.length && heap[r][0] > heap[m][0]) m = r;
      if (m === i) return;
      [heap[i], heap[m]] = [heap[m], heap[i]];
      i = m;
    }
  };
  const order = [...buildings].sort((a, b) => a[0] - b[0]);
  const xs = [...new Set(buildings.flatMap((b) => [b[0], b[1]]))].sort((a, b) => a - b);
  const out: number[][] = [];
  let i = 0;
  let last = 0;
  for (const x of xs) {
    while (i < order.length && order[i][0] <= x) {
      push([order[i][2], order[i][1]]);
      i++;
    }
    while (heap.length > 0 && heap[0][1] <= x) pop();
    const h = heap.length > 0 ? heap[0][0] : 0;
    if (h !== last) {
      out.push([x, h]);
      last = h;
    }
  }
  return out;
}
