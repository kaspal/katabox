export function tillTotal(prices: string[]): number {
  // Count in whole cents: "12.30" is 1230.
  return prices.reduce((sum, p) => sum + Number(p.replace(".", "")), 0);
}
