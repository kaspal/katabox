const MOD = 1000000007;

export function tilingsApart(n: number): number {
  const a = new Array<number>(n + 1).fill(0);
  const b = new Array<number>(n + 1).fill(0);
  a[0] = 1;
  for (let i = 1; i <= n; i++) {
    let v = a[i - 1] + b[i - 1];
    if (i >= 2) v += a[i - 2] + b[i - 2];
    a[i] = v % MOD;
    if (i >= 3) b[i] = a[i - 3];
  }
  return (a[n] + b[n]) % MOD;
}
