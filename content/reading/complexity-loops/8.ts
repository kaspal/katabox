function f(n: number): number {
  let evens = 0;
  for (let i = 0; i < n; i++) {
    if (i % 2 === 0) {
      evens++;
    }
  }
  let total = 0;
  for (let j = 0; j < 2 * n; j++) {
    total += j % 3;
  }
  return evens + total;
}
