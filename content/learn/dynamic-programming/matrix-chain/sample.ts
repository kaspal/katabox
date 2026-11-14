function matrixChain(dims: number[]): number[] {
  const n = dims.length - 1;
  const cost = Array.from({ length: n }, () => new Array<number>(n).fill(0));
  for (let span = 2; span <= n; span++) {
    for (let i = 0; i + span - 1 < n; i++) {
      const j = i + span - 1;
      cost[i][j] = 1e9;
      for (let k = i; k < j; k++) {
        const total = cost[i][k] + cost[k + 1][j] + dims[i] * dims[k + 1] * dims[j + 1];
        if (total < cost[i][j]) {
          cost[i][j] = total; // @step split
        }
      }
    }
  }
  return [cost[0][n - 1]];
}
