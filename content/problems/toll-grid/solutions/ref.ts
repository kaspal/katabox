export function cheapestCrossing(tolls: number[][]): number {
  const rows = tolls.length;
  const cols = tolls[0].length;
  const cells = rows * cols;
  // State s = used * cells + r * cols + c.
  const cost = new Float64Array(2 * cells).fill(Infinity);
  // A binary min-heap of [cost, state] pairs.
  const heap: [number, number][] = [];
  const push = (d: number, s: number) => {
    heap.push([d, s]);
    let i = heap.length - 1;
    while (i > 0) {
      const p = (i - 1) >> 1;
      if (heap[p][0] <= heap[i][0]) break;
      [heap[p], heap[i]] = [heap[i], heap[p]];
      i = p;
    }
  };
  const pop = (): [number, number] => {
    const top = heap[0];
    const last = heap.pop()!;
    if (heap.length > 0) {
      heap[0] = last;
      let i = 0;
      for (;;) {
        const l = 2 * i + 1;
        const r = l + 1;
        let m = i;
        if (l < heap.length && heap[l][0] < heap[m][0]) m = l;
        if (r < heap.length && heap[r][0] < heap[m][0]) m = r;
        if (m === i) break;
        [heap[i], heap[m]] = [heap[m], heap[i]];
        i = m;
      }
    }
    return top;
  };
  const start = tolls[0][0];
  cost[0] = start;
  cost[cells] = Math.floor(start / 2);
  push(start, 0);
  push(Math.floor(start / 2), cells);
  const dr = [-1, 1, 0, 0];
  const dc = [0, 0, -1, 1];
  while (heap.length > 0) {
    const [d, s] = pop();
    if (d > cost[s]) continue;
    const used = s >= cells ? 1 : 0;
    const cell = s - used * cells;
    const r = Math.floor(cell / cols);
    const c = cell % cols;
    for (let k = 0; k < 4; k++) {
      const nr = r + dr[k];
      const nc = c + dc[k];
      if (nr < 0 || nr >= rows || nc < 0 || nc >= cols) continue;
      const toll = tolls[nr][nc];
      const t = used * cells + nr * cols + nc;
      if (d + toll < cost[t]) {
        cost[t] = d + toll;
        push(d + toll, t);
      }
      if (!used) {
        const u = cells + nr * cols + nc;
        const half = d + Math.floor(toll / 2);
        if (half < cost[u]) {
          cost[u] = half;
          push(half, u);
        }
      }
    }
  }
  return Math.min(cost[cells - 1], cost[2 * cells - 1]);
}
