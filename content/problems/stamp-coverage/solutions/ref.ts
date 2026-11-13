export function busiestCells(rows: number, cols: number, stamps: number[][]): number[] {
  const w = cols + 1;
  const d = new Int32Array((rows + 1) * w);
  for (const [t, l, b, r] of stamps) {
    d[t * w + l]++;
    d[t * w + r + 1]--;
    d[(b + 1) * w + l]--;
    d[(b + 1) * w + r + 1]++;
  }
  let most = 0;
  let cells = 0;
  const above = new Int32Array(cols);
  for (let i = 0; i < rows; i++) {
    let run = 0;
    for (let j = 0; j < cols; j++) {
      run += d[i * w + j];
      const v = above[j] + run;
      above[j] = v;
      if (v > most) {
        most = v;
        cells = 1;
      } else if (v === most) {
        cells++;
      }
    }
  }
  return [most, cells];
}
