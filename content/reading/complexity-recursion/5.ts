function f(n: number): number {
  return power(3, n, 1_000_007);
}

function power(base: number, e: number, mod: number): number {
  if (e === 0) {
    return 1;
  }
  const half = power(base, Math.floor(e / 2), mod);
  let result = (half * half) % mod;
  if (e % 2 === 1) {
    result = (result * base) % mod;
  }
  return result;
}
