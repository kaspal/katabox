export function longestStretch(costs: number[], budget: number): number {
  let best = 0;
  let left = 0;
  let total = 0;
  for (let right = 0; right < costs.length; right++) {
    total += costs[right];
    while (total > budget) {
      total -= costs[left];
      left++;
    }
    best = Math.max(best, right - left + 1);
  }
  return best;
}
