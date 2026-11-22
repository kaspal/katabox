export function distinctPairs(prices: number[], budget: number): number {
  let i = 0;
  let j = prices.length - 1;
  let count = 0;
  while (i < j) {
    const s = prices[i] + prices[j];
    if (s < budget) i++;
    else if (s > budget) j--;
    else {
      count++;
      i++;
      j--;
    }
  }
  return count;
}
