export function swapsToSort(heights: number[]): number {
  // Bottom-up merge sort: merge runs of width 1, 2, 4, … between two buffers.
  const n = heights.length;
  let a = Float64Array.from(heights);
  let b = new Float64Array(n);
  let count = 0;
  for (let width = 1; width < n; width *= 2) {
    for (let lo = 0; lo < n; lo += 2 * width) {
      const mid = Math.min(lo + width, n);
      const hi = Math.min(lo + 2 * width, n);
      let i = lo;
      let j = mid;
      let k = lo;
      while (i < mid && j < hi) {
        if (a[i] <= a[j]) b[k++] = a[i++];
        else {
          b[k++] = a[j++];
          count += mid - i;
        }
      }
      while (i < mid) b[k++] = a[i++];
      while (j < hi) b[k++] = a[j++];
    }
    [a, b] = [b, a];
  }
  return count;
}
