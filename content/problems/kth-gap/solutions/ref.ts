export function kthGap(posts: number[], k: number): number {
  const a = Float64Array.from(posts).sort();
  const n = a.length;
  let lo = 0;
  let hi = a[n - 1] - a[0];
  while (lo < hi) {
    const mid = Math.floor((lo + hi) / 2);
    let count = 0;
    let i = 0;
    for (let j = 0; j < n; j++) {
      while (a[j] - a[i] > mid) i++;
      count += j - i;
    }
    if (count >= k) hi = mid;
    else lo = mid + 1;
  }
  return lo;
}
