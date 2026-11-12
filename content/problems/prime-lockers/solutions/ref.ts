export function primeCounts(limits: number[]): number[] {
  let m = 0;
  for (const x of limits) if (x > m) m = x;
  const composite = new Uint8Array(m + 1);
  for (let p = 2; p * p <= m; p++) {
    if (composite[p]) continue;
    for (let q = p * p; q <= m; q += p) composite[q] = 1;
  }
  const count = new Int32Array(m + 1);
  for (let x = 2; x <= m; x++) count[x] = count[x - 1] + (composite[x] ? 0 : 1);
  return limits.map((x) => count[x]);
}
