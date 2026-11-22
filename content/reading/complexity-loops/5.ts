function f(n: number): number {
  let power = 1;
  let steps = 0;
  while (power <= n) {
    power *= 3;
    steps++;
  }
  return steps;
}
