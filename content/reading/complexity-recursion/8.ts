function f(n: number): number {
  if (n <= 1) {
    return 0;
  }
  let work = 0;
  for (let i = 0; i < n; i++) {
    work++;
  }
  return work + f(Math.floor(n / 2));
}
