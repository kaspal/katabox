export function withinFixes(typed: string, meant: string, k: number): boolean {
  const n = typed.length;
  const m = meant.length;
  if (Math.abs(n - m) > k) return false;
  const big = k + 1;
  const w = 2 * k + 1;
  // Band row: column j lives at index j - i + k.
  let prev = new Array<number>(w).fill(big);
  for (let j = 0; j <= Math.min(m, k); j++) prev[j + k] = j;
  for (let i = 1; i <= n; i++) {
    const cur = new Array<number>(w).fill(big);
    const lo = Math.max(0, i - k);
    const hi = Math.min(m, i + k);
    for (let j = lo; j <= hi; j++) {
      const d = j - i + k;
      if (j === 0) {
        cur[d] = Math.min(i, big);
        continue;
      }
      let best = prev[d] + (typed[i - 1] === meant[j - 1] ? 0 : 1); // (i-1, j-1)
      if (d + 1 < w) best = Math.min(best, prev[d + 1] + 1); // (i-1, j)
      if (d - 1 >= 0) best = Math.min(best, cur[d - 1] + 1); // (i, j-1)
      cur[d] = Math.min(best, big);
    }
    prev = cur;
  }
  return prev[m - n + k] <= k;
}
