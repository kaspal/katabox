export function settle(paid: number[]): number[] {
  let total = 0;
  for (const p of paid) total += p;
  const share = total / paid.length;
  return paid.map((p) => p - share);
}
