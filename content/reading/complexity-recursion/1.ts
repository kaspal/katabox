function f(n: number): number {
  if (n === 0) {
    return 0;
  }
  return n + f(n - 1);
}
