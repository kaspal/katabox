export function countBelow(codes: number[], queries: number[]): number[] {
  return queries.map((x) => {
    let lo = 0;
    let hi = codes.length;
    while (lo < hi) {
      const mid = (lo + hi) >> 1;
      if (codes[mid] < x) lo = mid + 1;
      else hi = mid;
    }
    return lo;
  });
}
