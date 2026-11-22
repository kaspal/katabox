function f(n: number): number {
  for (let i = 0; i < n; i++) {
    if (2 * i >= n) {
      return i;
    }
  }
  return -1;
}
