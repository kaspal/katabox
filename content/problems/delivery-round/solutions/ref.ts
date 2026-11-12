export function shortestRound(dist: number[][]): number {
  const n = dist.length;
  const full = (1 << n) - 1;
  // best[mask * n + last]: fewest minutes to finish from (mask, last); filled from
  // the fullest masks down, since a mask only depends on larger ones.
  const best = new Float64Array((full + 1) * n).fill(Infinity);
  for (let last = 0; last < n; last++) best[full * n + last] = dist[last][0];
  for (let mask = full - 2; mask >= 1; mask -= 2) {
    // Only masks containing the depot (odd masks) are used.
    for (let last = 0; last < n; last++) {
      if (!((mask >> last) & 1)) continue;
      let b = Infinity;
      for (let j = 0; j < n; j++) {
        if ((mask >> j) & 1) continue;
        const v = dist[last][j] + best[(mask | (1 << j)) * n + j];
        if (v < b) b = v;
      }
      best[mask * n + last] = b;
    }
  }
  return n === 1 ? 0 : best[1 * n + 0];
}
