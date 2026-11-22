export function risingRun(heights: number[]): number {
  const tails: number[] = [];
  for (const h of heights) {
    // The first tail >= h: an equal height replaces it, it never extends.
    let lo = 0;
    let hi = tails.length;
    while (lo < hi) {
      const mid = (lo + hi) >> 1;
      if (tails[mid] < h) lo = mid + 1;
      else hi = mid;
    }
    tails[lo] = h;
  }
  return tails.length;
}
