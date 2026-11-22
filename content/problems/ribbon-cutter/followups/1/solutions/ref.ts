export function longestPieces(ribbons: number[], orders: number[]): number[] {
  let m = 0;
  for (const r of ribbons) if (r > m) m = r;
  const n = ribbons.length;
  const atMost = new Array<number>(m + 1).fill(0);
  for (const r of ribbons) atMost[r]++;
  for (let v = 1; v <= m; v++) atMost[v] += atMost[v - 1];
  const count = new Array<number>(m + 1).fill(0);
  for (let len = 1; len <= m; len++) {
    let total = 0;
    for (let k = len; k <= m; k += len) total += n - atMost[k - 1];
    count[len] = total;
  }
  return orders.map((need) => {
    // The largest L with count[L] >= need (count is non-increasing), 0 when none.
    let lo = 0;
    let hi = m;
    while (lo < hi) {
      const mid = (lo + hi + 1) >> 1;
      if (count[mid] >= need) lo = mid;
      else hi = mid - 1;
    }
    return lo;
  });
}
