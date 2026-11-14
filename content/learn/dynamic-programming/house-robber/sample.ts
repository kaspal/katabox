function rob(values: number[]): number[] {
  const n = values.length;
  const best = new Array<number>(n + 1).fill(0);
  best[1] = values[0];
  for (let i = 2; i <= n; i++) {
    const skip = best[i - 1];
    const take = best[i - 2] + values[i - 1];
    best[i] = Math.max(skip, take); // @step choose
  }
  const chosen: number[] = [];
  let i = n;
  while (i > 0) {
    if (best[i] === best[i - 1]) {
      i -= 1;
    } else {
      chosen.push(i - 1); // @step trace
      i -= 2;
    }
  }
  return [best[n], ...chosen.reverse()];
}
