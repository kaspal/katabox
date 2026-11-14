function powerMod(base: number, exp: number, mod: number): number[] {
  let result = 1 % mod;
  base %= mod;
  while (exp > 0) {
    if (exp & 1) {
      result = (result * base) % mod; // @step multiply
    }
    base = (base * base) % mod; // @step square
    exp >>= 1;
  }
  return [result];
}
