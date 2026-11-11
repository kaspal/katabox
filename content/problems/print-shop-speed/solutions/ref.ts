export function minSpeed(pages: number[], hours: number): number {
  let lo = 1;
  let hi = Math.max(...pages);
  while (lo < hi) {
    const mid = Math.floor((lo + hi) / 2);
    let need = 0;
    for (const p of pages) need += Math.floor((p + mid - 1) / mid);
    if (need <= hours) hi = mid;
    else lo = mid + 1;
  }
  return lo;
}
