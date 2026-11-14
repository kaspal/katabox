function pascalRow(n: number): number[] {
  const rows = [[1]];
  for (let i = 1; i <= n; i++) {
    const row = new Array<number>(i + 1).fill(1);
    for (let k = 1; k < i; k++) {
      row[k] = rows[i - 1][k - 1] + rows[i - 1][k]; // @step add
    }
    rows.push(row);
  }
  return rows[n];
}
