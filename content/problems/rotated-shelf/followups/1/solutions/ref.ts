export function findLabels(shelf: number[], labels: number[]): number[] {
  const n = shelf.length;
  let lo = 0;
  let hi = n - 1;
  while (lo < hi) {
    const m = (lo + hi) >> 1;
    if (shelf[m] > shelf[hi]) lo = m + 1;
    else hi = m;
  }
  const start = lo;
  const at = (k: number) => shelf[(k + start) % n];
  return labels.map((x) => {
    let a = 0;
    let b = n;
    while (a < b) {
      const m = (a + b) >> 1;
      if (at(m) < x) a = m + 1;
      else b = m;
    }
    return a < n && at(a) === x ? (a + start) % n : -1;
  });
}
