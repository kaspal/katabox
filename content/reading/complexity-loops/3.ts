function f(n: number): number {
  let total = 0;
  for (let i = 0; i < n; i += 3) {
    total += i * i;
  }
  return total;
}
