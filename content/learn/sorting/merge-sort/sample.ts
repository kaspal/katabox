function mergeSort(a: number[], lo: number, hi: number): void {
  if (lo >= hi) return; // @step base
  const mid = Math.floor((lo + hi) / 2); // @step split
  mergeSort(a, lo, mid);
  mergeSort(a, mid + 1, hi);
  merge(a, lo, mid, hi);
}

function merge(a: number[], lo: number, mid: number, hi: number): void {
  const buffer = a.slice(lo, hi + 1); // @step copy
  let i = 0;
  let j = mid + 1 - lo;
  for (let k = lo; k <= hi; k++) {
    if (j > hi - lo || (i <= mid - lo && buffer[i] <= buffer[j])) { // @step pick
      a[k] = buffer[i]; // @step take-left
      i++;
    } else {
      a[k] = buffer[j]; // @step take-right
      j++;
    }
  }
}
