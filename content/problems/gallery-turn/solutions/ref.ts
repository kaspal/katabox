export function turnRight(grid: number[][]): number[][] {
  const r = grid.length;
  const c = grid[0].length;
  const out: number[][] = [];
  for (let j = 0; j < c; j++) {
    const row: number[] = new Array(r);
    for (let i = 0; i < r; i++) row[i] = grid[r - 1 - i][j];
    out.push(row);
  }
  return out;
}
