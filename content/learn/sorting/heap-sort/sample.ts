function heapSort(a: number[]): void {
  const n = a.length;
  for (let start = Math.floor(n / 2) - 1; start >= 0; start--) {
    siftDown(a, start, n); // @step build
  }
  for (let end = n - 1; end > 0; end--) {
    [a[0], a[end]] = [a[end], a[0]]; // @step extract
    siftDown(a, 0, end);
  }
}

function siftDown(a: number[], root: number, size: number): void {
  let child = 2 * root + 1;
  while (child < size) {
    if (child + 1 < size && a[child + 1] > a[child]) { // @step pick
      child++;
    }
    if (a[root] >= a[child]) return; // @step compare
    [a[root], a[child]] = [a[child], a[root]]; // @step swap
    root = child;
    child = 2 * root + 1;
  }
}
