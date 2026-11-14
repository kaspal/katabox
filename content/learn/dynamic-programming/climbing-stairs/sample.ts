function climb(n: number): number[] {
  const ways = new Array<number>(n + 1).fill(0);
  ways[0] = 1;
  for (let i = 1; i <= n; i++) {
    ways[i] = ways[i - 1]; // @step one
    if (i >= 2) {
      ways[i] += ways[i - 2]; // @step two
    }
  }
  return ways;
}
