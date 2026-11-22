function f(n: number): number {
  let lo = 0;
  let hi = n - 1;
  const target = Math.floor(n / 2);
  let pairs = 0;
  while (lo < hi) {
    const s = lo + hi;
    if (s < target) {
      lo++;
    } else if (s > target) {
      hi--;
    } else {
      pairs++;
      lo++;
      hi--;
    }
  }
  return pairs;
}
