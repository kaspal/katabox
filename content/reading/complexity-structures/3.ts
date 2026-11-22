function f(n: number): number {
  const evens: number[] = [];
  for (let i = 0; i < n; i++) {
    evens.push(2 * i);
  }
  let total = 0;
  for (let i = 0; i < n; i++) {
    const q = 2 * i + 1;
    let lo = 0;
    let hi = n;
    while (lo < hi) {
      const mid = Math.floor((lo + hi) / 2);
      if (evens[mid] < q) {
        lo = mid + 1;
      } else {
        hi = mid;
      }
    }
    total += lo;
  }
  return total;
}
