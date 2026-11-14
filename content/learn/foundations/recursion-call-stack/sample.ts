function total(a: number[], lo: number, hi: number): number {
  if (lo === hi) { // @step check
    return a[lo]; // @step leaf
  }
  const mid = Math.floor((lo + hi) / 2); // @step split
  const left = total(a, lo, mid);
  const right = total(a, mid + 1, hi);
  return left + right; // @step combine
}
