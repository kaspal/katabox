export function loneSock(ids: number[]): number {
  // Values fit in 31 bits, so the 32-bit XOR of JavaScript is exact.
  let acc = 0;
  for (const x of ids) acc ^= x;
  return acc;
}
