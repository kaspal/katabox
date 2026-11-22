function f(n: number): number {
  if (n <= 1) {
    return 0;
  }
  return 1 + f(Math.floor(n / 2));
}
