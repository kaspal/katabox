export function gentlestClimb(heights: number[][]): number {
  const rows = heights.length;
  const cols = heights[0].length;
  const n = rows * cols;
  const flat = new Int32Array(n);
  for (let r = 0; r < rows; r++) for (let c = 0; c < cols; c++) flat[r * cols + c] = heights[r][c];
  const best = new Float64Array(n).fill(Infinity);
  best[0] = 0;
  // A binary min-heap of [effort, spot] pairs, stored flat.
  const hd: number[] = [];
  const hi: number[] = [];
  const swap = (a: number, b: number) => {
    [hd[a], hd[b]] = [hd[b], hd[a]];
    [hi[a], hi[b]] = [hi[b], hi[a]];
  };
  const push = (d: number, i: number) => {
    hd.push(d);
    hi.push(i);
    let k = hd.length - 1;
    while (k > 0) {
      const p = (k - 1) >> 1;
      if (hd[p] <= hd[k]) break;
      swap(p, k);
      k = p;
    }
  };
  const pop = (): [number, number] => {
    const top: [number, number] = [hd[0], hi[0]];
    const ld = hd.pop()!;
    const li = hi.pop()!;
    if (hd.length > 0) {
      hd[0] = ld;
      hi[0] = li;
      let k = 0;
      for (;;) {
        const l = 2 * k + 1;
        const r = l + 1;
        let m = k;
        if (l < hd.length && hd[l] < hd[m]) m = l;
        if (r < hd.length && hd[r] < hd[m]) m = r;
        if (m === k) break;
        swap(k, m);
        k = m;
      }
    }
    return top;
  };
  push(0, 0);
  while (hd.length > 0) {
    const [d, i] = pop();
    if (i === n - 1) return d;
    if (d > best[i]) continue;
    const r = Math.floor(i / cols);
    const c = i - r * cols;
    const visit = (j: number) => {
      const nd = Math.max(d, Math.abs(flat[j] - flat[i]));
      if (nd < best[j]) {
        best[j] = nd;
        push(nd, j);
      }
    };
    if (r > 0) visit(i - cols);
    if (r + 1 < rows) visit(i + cols);
    if (c > 0) visit(i - 1);
    if (c + 1 < cols) visit(i + 1);
  }
  return 0;
}
