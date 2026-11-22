export function bestPackHuge(weights: number[], values: number[], capacity: number): number {
  let total = 0;
  for (const v of values) total += v;
  const lightest = new Array<number>(total + 1).fill(Infinity);
  lightest[0] = 0;
  for (let i = 0; i < weights.length; i++) {
    const w = weights[i];
    const v = values[i];
    for (let t = total; t >= v; t--) if (lightest[t - v] + w < lightest[t]) lightest[t] = lightest[t - v] + w;
  }
  for (let t = total; t >= 0; t--) if (lightest[t] <= capacity) return t;
  return 0;
}
