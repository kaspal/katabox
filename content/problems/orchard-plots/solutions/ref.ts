export function plotTotals(grid: number[][], queries: number[][]): number[] {
  const rows = grid.length;
  const cols = grid[0].length;
  const w = cols + 1;
  const pre = new Float64Array((rows + 1) * w);
  for (let r = 0; r < rows; r++) {
    let run = 0;
    for (let c = 0; c < cols; c++) {
      run += grid[r][c];
      pre[(r + 1) * w + c + 1] = pre[r * w + c + 1] + run;
    }
  }
  return queries.map(([t, l, b, r]) =>
    pre[(b + 1) * w + r + 1] - pre[t * w + r + 1] - pre[(b + 1) * w + l] + pre[t * w + l]);
}
