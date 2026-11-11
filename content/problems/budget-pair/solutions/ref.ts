export function bestPair(prices: number[], budget: number): number {
  let lo = 0;
  let hi = prices.length - 1;
  let best = -1;
  while (lo < hi) {
    const total = prices[lo] + prices[hi];
    if (total > budget) {
      hi--;
    } else {
      if (total > best) best = total;
      lo++;
    }
  }
  return best;
}
