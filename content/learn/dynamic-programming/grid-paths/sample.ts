function gridPaths(rows: number, cols: number, cells: number[]): number[] {
  const ways = Array.from({ length: rows }, () =>
    new Array<number>(cols).fill(0),
  );
  for (let r = 0; r < rows; r++) {
    for (let c = 0; c < cols; c++) {
      if (cells[r * cols + c] === 1) {
        ways[r][c] = 0; // @step blocked
      } else if (r === 0 && c === 0) {
        ways[r][c] = 1;
      } else {
        const up = r > 0 ? ways[r - 1][c] : 0;
        const left = c > 0 ? ways[r][c - 1] : 0;
        ways[r][c] = up + left; // @step add
      }
    }
  }
  return [ways[rows - 1][cols - 1]];
}
