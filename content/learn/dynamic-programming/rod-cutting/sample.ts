function rodCutting(prices: number[]): number[] {
  const n = prices.length;
  const best = new Array<number>(n + 1).fill(0);
  const first = new Array<number>(n + 1).fill(0);
  for (let length = 1; length <= n; length++) {
    for (let cut = 1; cut <= length; cut++) {
      if (prices[cut - 1] + best[length - cut] > best[length]) {
        best[length] = prices[cut - 1] + best[length - cut]; // @step improve
        first[length] = cut;
      }
    }
  }
  const pieces: number[] = [];
  let length = n;
  while (length > 0) {
    pieces.push(first[length]); // @step trace
    length -= first[length];
  }
  return [best[n], ...pieces];
}
