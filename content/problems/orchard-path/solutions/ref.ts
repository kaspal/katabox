export function mostApples(grid: number[][]): number {
  const cols = grid[0].length;
  const best = new Array<number>(cols).fill(0);
  for (let r = 0; r < grid.length; r++) {
    const row = grid[r];
    for (let c = 0; c < cols; c++) {
      let prev: number;
      if (r === 0) prev = c > 0 ? best[c - 1] : 0;
      else if (c === 0) prev = best[0];
      else prev = Math.max(best[c], best[c - 1]);
      best[c] = row[c] + prev;
    }
  }
  return best[cols - 1];
}
