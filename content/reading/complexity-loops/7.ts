function f(n: number): number {
  let work = 0;
  let width = n;
  while (width > 1) {
    for (let i = 0; i < n; i++) {
      work++;
    }
    width = Math.floor(width / 2);
  }
  return work;
}
