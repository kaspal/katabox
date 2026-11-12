export function outfits(prices: number[][], budget: number): number {
  const groups = prices.map((g) => [...g].sort((a, b) => a - b));
  const cheapest = new Array<number>(groups.length + 1).fill(0);
  for (let c = groups.length - 1; c >= 0; c--) cheapest[c] = cheapest[c + 1] + groups[c][0];
  const choose = (c: number, spent: number): number => {
    if (c === groups.length) return 1;
    let total = 0;
    for (const price of groups[c]) {
      if (spent + price + cheapest[c + 1] > budget) break;
      total += choose(c + 1, spent + price);
    }
    return total;
  };
  return choose(0, 0);
}
