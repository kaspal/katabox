export function longestRise(heights: number[]): number {
  const tails: number[] = [];
  for (const h of heights) {
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
