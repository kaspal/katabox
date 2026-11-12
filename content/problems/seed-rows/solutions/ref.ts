export function plantings(length: number, widths: number[]): number {
  const MOD = 1_000_000_007;
  const k = widths.length;
  const ways = new Float64Array((length + 1) * k);
  const total = new Float64Array(length + 1);
  total[0] = 1;
  for (let x = 1; x <= length; x++) {
    let sum = 0;
    for (let kind = 0; kind < k; kind++) {
      const w = widths[kind];
      if (w <= x) {
        const v = (total[x - w] - ways[(x - w) * k + kind] + MOD) % MOD;
        ways[x * k + kind] = v;
        sum += v;
      }
    }
    total[x] = sum % MOD;
  }
  return total[length];
}
