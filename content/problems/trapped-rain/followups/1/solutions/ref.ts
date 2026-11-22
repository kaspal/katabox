export function trappedRainGrid(heights: number[][]): number {
  const rows = heights.length;
  const cols = heights[0].length;
  const seen = new Uint8Array(rows * cols);
  // A binary min-heap of [level, cell].
  const heap: [number, number][] = [];
  const push = (x: [number, number]) => {
    heap.push(x);
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
        [heap[m], heap[i]] = [heap[i], heap[m]];
        i = m;
      }
    }
    return top;
  };
  for (let r = 0; r < rows; r++)
    for (let c = 0; c < cols; c++)
      if (r === 0 || r === rows - 1 || c === 0 || c === cols - 1) {
        seen[r * cols + c] = 1;
        push([heights[r][c], r * cols + c]);
      }
  let water = 0;
  while (heap.length > 0) {
    const [level, cell] = pop();
    const r = Math.floor(cell / cols);
    const c = cell % cols;
    for (const [nr, nc] of [
      [r + 1, c],
      [r - 1, c],
      [r, c + 1],
      [r, c - 1],
    ]) {
      if (nr < 0 || nr >= rows || nc < 0 || nc >= cols || seen[nr * cols + nc]) continue;
      seen[nr * cols + nc] = 1;
      const h = heights[nr][nc];
      water += Math.max(0, level - h);
      push([Math.max(level, h), nr * cols + nc]);
    }
  }
  return water;
}
