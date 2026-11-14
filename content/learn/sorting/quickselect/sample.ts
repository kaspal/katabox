function quickselect(a: number[], k: number): number {
  let lo = 0;
  let hi = a.length - 1;
  while (true) {
    const p = partition(a, lo, hi);
    if (p === k - 1) return a[p]; // @step found
    if (p < k - 1) { // @step side
      lo = p + 1;
    } else {
      hi = p - 1;
    }
  }
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
