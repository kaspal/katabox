export function longestWithRefunds(costs: number[], budget: number): number {
  const starts: number[] = [0];
  const where: number[] = [0];
  let prefix = 0;
  let best = 0;
  for (let j = 1; j <= costs.length; j++) {
    prefix += costs[j - 1];
    const need = prefix - budget;
    let lo = 0;
    let hi = starts.length;
    while (lo < hi) {
      const m = (lo + hi) >> 1;
      if (starts[m] < need) lo = m + 1;
      else hi = m;
    }
    if (lo < starts.length) best = Math.max(best, j - where[lo]);
    if (prefix > starts[starts.length - 1]) {
      starts.push(prefix);
      where.push(j);
    }
  }
  return best;
}
