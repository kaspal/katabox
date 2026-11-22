function f(n: number): number {
  let total = 0;
  for (let k = 1; k <= 100; k++) {
    total += n % k;
  }
  return total;
}
