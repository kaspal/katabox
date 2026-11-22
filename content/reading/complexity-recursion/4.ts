function f(n: number): number {
  const memo = new Map<number, number>();
  function ways(k: number): number {
    if (k <= 1) {
      return 1;
    }
    const known = memo.get(k);
    if (known !== undefined) {
      return known;
    }
    const value = (ways(k - 1) + ways(k - 2)) % 1_000_000_007;
    memo.set(k, value);
    return value;
  }
  return ways(n);
}
