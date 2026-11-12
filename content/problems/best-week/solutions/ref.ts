export function bestWeek(sales: number[], k: number): number {
  let window = 0;
  for (let i = 0; i < k; i++) window += sales[i];
  let best = window;
  for (let i = k; i < sales.length; i++) {
    window += sales[i] - sales[i - k];
    if (window > best) best = window;
  }
  return best;
}
