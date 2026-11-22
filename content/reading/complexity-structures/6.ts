function f(n: number): number {
  const prefix: number[] = new Array(n + 1).fill(0);
  for (let i = 0; i < n; i++) {
    prefix[i + 1] = prefix[i] + (i % 10);
  }
  let best = 0;
  for (let q = 0; q < 2 * n; q++) {
    const left = q % n;
    const right = Math.min(n, left + (q % 50) + 1);
    best = Math.max(best, prefix[right] - prefix[left]);
  }
  return best;
}
