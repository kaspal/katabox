export function menuCombos(prices: number[], budget: number): number[][] {
  const n = prices.length;
  const out: number[][] = [];
  for (let mask = 1; mask < 1 << n; mask++) {
    let total = 0;
    const dishes: number[] = [];
    for (let i = 0; i < n; i++) {
      if ((mask >> i) & 1) {
        total += prices[i];
        dishes.push(i);
      }
    }
    if (total === budget) out.push(dishes);
  }
  out.sort((a, b) => {
    for (let i = 0; i < Math.min(a.length, b.length); i++) {
      if (a[i] !== b[i]) return a[i] - b[i];
    }
    return a.length - b.length;
  });
  return out;
}
