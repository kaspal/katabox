const RUN = 4;

function timSort(a: number[]): void {
  const n = a.length;
  for (let start = 0; start < n; start += RUN) {
    insertionSort(a, start, Math.min(start + RUN, n) - 1); // @step run
  }
  for (let width = RUN; width < n; width *= 2) {
    for (let lo = 0; lo < n - width; lo += 2 * width) {
      const mid = lo + width - 1;
      const hi = Math.min(lo + 2 * width, n) - 1;
      merge(a, lo, mid, hi); // @step merge
    }
  }
}

function insertionSort(a: number[], lo: number, hi: number): void {
  for (let i = lo + 1; i <= hi; i++) {
    const key = a[i]; // @step take
    let j = i - 1;
    while (j >= lo && a[j] > key) { // @step compare
      a[j + 1] = a[j]; // @step shift
      j--;
    }
    a[j + 1] = key; // @step insert
  }
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
