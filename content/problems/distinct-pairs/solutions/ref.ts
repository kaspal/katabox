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
      const left = prices[i];
      const right = prices[j];
      while (i < j && prices[i] === left) i++;
      while (i < j && prices[j] === right) j--;
    }
  }
  return count;
}
