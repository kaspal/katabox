export function freeNumbers(shelf: number[], queries: number[]): number[] {
  const n = shelf.length;
  let lo = 0;
  let hi = n - 1;
  while (lo < hi) {
    const mid = Math.floor((lo + hi) / 2);
    if (shelf[mid] > shelf[hi]) lo = mid + 1;
    else hi = mid;
  }
  const p = lo;
  const freeBelow = (i: number) => shelf[(p + i) % n] - (i + 1);
  const out: number[] = [];
  for (const k of queries) {
    let a = 0;
    let b = n;
    while (a < b) {
      const mid = Math.floor((a + b) / 2);
      if (freeBelow(mid) >= k) b = mid;
      else a = mid + 1;
    }
    out.push(k + a);
  }
  return out;
}
