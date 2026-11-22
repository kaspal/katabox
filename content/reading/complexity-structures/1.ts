function f(n: number): number {
  const seen = new Set<number>();
  let repeats = 0;
  for (let i = 0; i < n; i++) {
    const key = (i * i) % n;
    if (seen.has(key)) {
      repeats++;
    } else {
      seen.add(key);
    }
  }
  return repeats;
}
