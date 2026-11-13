export function peakIndex(heights: number[]): number {
  let lo = 0;
  let hi = heights.length - 1;
  while (lo < hi) {
    const m = (lo + hi) >> 1;
    if (heights[m] < heights[m + 1]) lo = m + 1;
    else hi = m;
  }
  return lo;
}
