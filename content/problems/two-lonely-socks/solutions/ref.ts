export function twoLonely(ids: number[]): number[] {
  // Ids fit in 31 bits, so JavaScript's 32-bit bit operators are exact.
  let t = 0;
  for (const x of ids) t ^= x;
  const bit = t & -t;
  let a = 0;
  for (const x of ids) if (x & bit) a ^= x;
  const b = t ^ a;
  return a < b ? [a, b] : [b, a];
}
