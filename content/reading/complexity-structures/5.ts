function f(n: number): number {
  const grid: number[][] = [];
  for (let i = 0; i < n; i++) {
    const row: number[] = [];
    for (let j = 0; j < n; j++) {
      row.push((i * j) % 10);
    }
    grid.push(row);
  }
  return grid[n - 1][n - 1];
}
