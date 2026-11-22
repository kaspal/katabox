function f(n: number): number {
  let count = 0;
  for (let i = 0; i < n; i++) {
    for (let j = 0; j < i; j++) {
      count++;
    }
  }
  return count;
}
