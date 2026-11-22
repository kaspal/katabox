export function tillTotal(prices: string[]): number {
  let total = 0;
  for (const p of prices) total += parseFloat(p);
  return Math.floor(total * 100);
}
