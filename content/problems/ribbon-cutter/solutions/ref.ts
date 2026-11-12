export function longestPiece(ribbons: number[], pieces: number): number {
  const count = (len: number): number => {
    let c = 0;
    for (const r of ribbons) c += Math.floor(r / len);
    return c;
  };
  if (count(1) < pieces) return 0;
  let lo = 1;
  let hi = 0;
  for (const r of ribbons) if (r > hi) hi = r;
  while (lo < hi) {
    const mid = Math.floor((lo + hi + 1) / 2);
    if (count(mid) >= pieces) lo = mid;
    else hi = mid - 1;
  }
  return lo;
}
