export function tilings(n: number): number {
  const MOD = 1_000_000_007;
  let a = 0;
  let b = 0;
  let c = 1;
  for (let i = 0; i < n; i++) {
    const next = (a + b + c) % MOD;
    a = b;
    b = c;
    c = next;
  }
  return c;
}
