// Two primes below 2^26, so every product stays exact in a JavaScript number.
const M1 = 50331653;
const M2 = 25165843;
const B = 131;

export function longestRepeat(text: string): number {
  const n = text.length;
  const h1 = new Float64Array(n + 1);
  const h2 = new Float64Array(n + 1);
  const p1 = new Float64Array(n + 1).fill(1);
  const p2 = new Float64Array(n + 1).fill(1);
  for (let i = 0; i < n; i++) {
    const c = text.charCodeAt(i);
    h1[i + 1] = (h1[i] * B + c) % M1;
    h2[i + 1] = (h2[i] * B + c) % M2;
    p1[i + 1] = (p1[i] * B) % M1;
    p2[i + 1] = (p2[i] * B) % M2;
  }
  const repeats = (len: number): boolean => {
    const seen = new Set<number>();
    for (let i = 0; i + len <= n; i++) {
      const a = (((h1[i + len] - h1[i] * p1[len]) % M1) + M1) % M1;
      const b = (((h2[i + len] - h2[i] * p2[len]) % M2) + M2) % M2;
      const key = a * M2 + b;
      if (seen.has(key)) return true;
      seen.add(key);
    }
    return false;
  };
  let lo = 0;
  let hi = n - 1;
  while (lo < hi) {
    const mid = Math.floor((lo + hi + 1) / 2);
    if (repeats(mid)) lo = mid;
    else hi = mid - 1;
  }
  return lo;
}
