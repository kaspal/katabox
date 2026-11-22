function knapsack(capacity: number, items: number[][]): number[] {
  const n = items.length;
  const best = Array.from({ length: n + 1 }, () =>
    new Array<number>(capacity + 1).fill(0),
  );
  for (let i = 1; i <= n; i++) {
    const [weight, value] = items[i - 1];
    for (let c = 0; c <= capacity; c++) {
      best[i][c] = best[i - 1][c]; // @step skip
      if (weight <= c && best[i - 1][c - weight] + value > best[i][c]) {
        best[i][c] = best[i - 1][c - weight] + value; // @step take
      }
    }
  }
  const chosen: number[] = [];
  let c = capacity;
  for (let i = n; i > 0; i--) {
    if (best[i][c] !== best[i - 1][c]) {
      chosen.push(i - 1); // @step trace
      c -= items[i - 1][0];
    }
  }
  return [best[n][capacity], ...chosen.reverse()];
}
