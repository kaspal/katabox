function f(n: number): number {
  if (n === 0) {
    return 0;
  }
  let total = 0;
  for (let i = 0; i < n; i++) {
    total += i;
  }
  return total + f(n - 1);
}
