function f(n: number): number {
  const a: number[] = [];
  const b: number[] = [];
  for (let i = 0; i < n; i++) {
    a.push(2 * i);
    b.push(3 * i);
  }
  let common = 0;
  for (const x of a) {
    let j = 0;
    while (j < b.length && b[j] !== x) {
      j++;
    }
    if (j < b.length) {
      common++;
    }
  }
  return common;
}
