export function steepDrops(prices: number[]): number {
  const a = Float64Array.from(prices);
  const buf = new Float64Array(a.length);
  const sort = (lo: number, hi: number): number => {
    if (hi - lo < 2) return 0;
    const mid = (lo + hi) >> 1;
    let count = sort(lo, mid) + sort(mid, hi);
    let j = mid;
    for (let i = lo; i < mid; i++) {
      while (j < hi && a[i] > 2 * a[j]) j++;
      count += j - mid;
    }
    let i = lo;
    let k = mid;
    let o = lo;
    while (i < mid && k < hi) buf[o++] = a[i] <= a[k] ? a[i++] : a[k++];
    while (i < mid) buf[o++] = a[i++];
    while (k < hi) buf[o++] = a[k++];
    a.set(buf.subarray(lo, hi), lo);
    return count;
  };
  return sort(0, a.length);
}
