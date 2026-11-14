function quickSort(a: number[], lo: number, hi: number): void {
  if (lo >= hi) return; // @step base
  const p = partition(a, lo, hi);
  quickSort(a, lo, p - 1);
  quickSort(a, p + 1, hi);
}

function partition(a: number[], lo: number, hi: number): number {
  const pivot = a[hi]; // @step pivot
  let i = lo;
  for (let j = lo; j < hi; j++) {
    if (a[j] < pivot) { // @step compare
      [a[i], a[j]] = [a[j], a[i]]; // @step swap
      i++;
    }
  }
  [a[i], a[hi]] = [a[hi], a[i]]; // @step place
  return i;
}
