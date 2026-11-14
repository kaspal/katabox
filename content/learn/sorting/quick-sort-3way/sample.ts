function quickSort3Way(a: number[], lo: number, hi: number): void {
  if (lo >= hi) return; // @step base
  const pivot = a[lo]; // @step pivot
  let lt = lo;
  let i = lo + 1;
  let gt = hi;
  while (i <= gt) {
    if (a[i] < pivot) { // @step compare
      [a[lt], a[i]] = [a[i], a[lt]]; // @step less
      lt++;
      i++;
    } else if (a[i] > pivot) {
      [a[i], a[gt]] = [a[gt], a[i]]; // @step greater
      gt--;
    } else {
      i++; // @step equal
    }
  }
  quickSort3Way(a, lo, lt - 1);
  quickSort3Way(a, gt + 1, hi);
}
