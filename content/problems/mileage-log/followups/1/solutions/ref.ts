export function fleetTotals(miles: number[][], queries: number[][]): number[] {
  const rows = miles.length;
  const cols = miles[0].length;
  const p: number[][] = Array.from({ length: rows + 1 }, () => new Array<number>(cols + 1).fill(0));
  for (let r = 0; r < rows; r++)
    for (let c = 0; c < cols; c++) p[r + 1][c + 1] = miles[r][c] + p[r][c + 1] + p[r + 1][c] - p[r][c];
  return queries.map(([v1, d1, v2, d2]) => p[v2 + 1][d2 + 1] - p[v1][d2 + 1] - p[v2 + 1][d1] + p[v1][d1]);
}
