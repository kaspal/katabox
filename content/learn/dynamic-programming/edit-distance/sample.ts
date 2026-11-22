function editDistance(a: string, b: string): number[] {
  const m = a.length;
  const n = b.length;
  const cost = Array.from({ length: m + 1 }, () =>
    new Array<number>(n + 1).fill(0),
  );
  for (let i = 0; i <= m; i++) cost[i][0] = i;
  for (let j = 0; j <= n; j++) cost[0][j] = j;
  for (let i = 1; i <= m; i++) {
    for (let j = 1; j <= n; j++) {
      if (a[i - 1] === b[j - 1]) {
        cost[i][j] = cost[i - 1][j - 1]; // @step same
      } else {
        cost[i][j] = // @step change
          1 + Math.min(cost[i - 1][j - 1], cost[i - 1][j], cost[i][j - 1]);
      }
    }
  }
  return [cost[m][n]];
}
