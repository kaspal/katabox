function f(n: number): number {
  return power(3, n, 1_000_007);
}

function power(base: number, e: number, mod: number): number {
  if (e === 0) {
    return 1;
  }
  let result = (power(base, Math.floor(e / 2), mod) * power(base, Math.floor(e / 2), mod)) % mod;
  if (e % 2 === 1) {
    result = (result * base) % mod;
  }
  return result;
}
