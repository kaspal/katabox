function f(n: number): number {
  let pairs = 0;
  for (let i = 0; i < n; i++) {
    for (let j = i + 1; j < n; j++) {
      if ((i + j) % 3 === 0) {
        pairs++;
      }
    }
  }
  return pairs;
}
