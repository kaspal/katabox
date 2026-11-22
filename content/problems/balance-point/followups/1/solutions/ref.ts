export function twoPivots(weights: number[]): number {
  const n = weights.length;
  const prefix = new Array<number>(n + 1).fill(0);
  for (let k = 0; k < n; k++) prefix[k + 1] = prefix[k] + weights[k];
  const total = prefix[n];
  const seen = new Map<string, number>();
  let count = 0;
  for (let j = 1; j < n; j++) {
    const i = j - 1;
    const key = `${prefix[i]},${weights[i]}`;
    seen.set(key, (seen.get(key) ?? 0) + 1);
    const right = total - prefix[j + 1];
    count += seen.get(`${right},${prefix[j] - 2 * right}`) ?? 0;
  }
  return count;
}
