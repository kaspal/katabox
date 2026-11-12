export function closestTrio(prices: number[], target: number): number {
  const p = Int32Array.from(prices).sort();
  const n = p.length;
  let best = p[0] + p[1] + p[2];
  let bestGap = Math.abs(best - target);
  for (let i = 0; i < n - 2; i++) {
    let lo = i + 1;
    let hi = n - 1;
    while (lo < hi) {
      const s = p[i] + p[lo] + p[hi];
      const gap = Math.abs(s - target);
      if (gap < bestGap || (gap === bestGap && s < best)) {
        best = s;
        bestGap = gap;
      }
      if (s < target) lo++;
      else if (s > target) hi--;
      else return s;
    }
  }
  return best;
}
