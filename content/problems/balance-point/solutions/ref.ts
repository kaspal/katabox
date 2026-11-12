export function balancePoint(weights: number[]): number {
  let total = 0;
  for (const w of weights) total += w;
  let left = 0;
  for (let i = 0; i < weights.length; i++) {
    if (left === total - left - weights[i]) return i;
    left += weights[i];
  }
  return -1;
}
