export function bestHaul(weights: number[], prices: number[], capacity: number): number {
  const order = weights.map((_, i) => i);
  order.sort((i, j) => prices[j] * weights[i] - prices[i] * weights[j]);
  let whole = 0;
  let room = capacity;
  for (const i of order) {
    if (weights[i] <= room) {
      whole += prices[i];
      room -= weights[i];
    } else {
      return whole + (prices[i] * room) / weights[i];
    }
  }
  return whole;
}
