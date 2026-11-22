export function bestProgramme(talks: number[][]): number {
  const order = [...talks].sort((a, b) => a[1] - b[1]);
  const n = order.length;
  const best = new Array<number>(n + 1).fill(0);
  for (let i = 0; i < n; i++) {
    const [s, , score] = order[i];
    // j = the number of talks among the first i ending at or before s.
    let lo = 0;
    let hi = i;
    while (lo < hi) {
      const m = (lo + hi) >> 1;
      if (order[m][1] <= s) lo = m + 1;
      else hi = m;
    }
    best[i + 1] = Math.max(best[i], best[lo] + score);
  }
  return best[n];
}
