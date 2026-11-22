function f(n: number): number {
  if (n <= 1) {
    return 1;
  }
  return n + 2 * f(n - 1);
}

console.log(f(3));
