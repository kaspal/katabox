const BASE = 26;
const MOD = 101;

function rabinKarp(text: string, pattern: string): number[] {
  const n = text.length;
  const m = pattern.length;
  if (m > n) return [0, 0];
  const code = (ch: string) => ch.charCodeAt(0) - 97;
  let high = 1;
  for (let k = 0; k < m - 1; k++) high = (high * BASE) % MOD;
  let target = 0;
  let window = 0;
  for (let k = 0; k < m; k++) {
    target = (target * BASE + code(pattern[k])) % MOD;
    window = (window * BASE + code(text[k])) % MOD;
  }
  const hits: number[] = [];
  let collisions = 0;
  for (let i = 0; i <= n - m; i++) {
    if (window === target) {
      if (text.slice(i, i + m) === pattern) {
        hits.push(i); // @step match
      } else {
        collisions++; // @step collision
      }
    }
    if (i + m < n) {
      // % can be negative in TypeScript: add MOD before the second %.
      const dropped = (((window - code(text[i]) * high) % MOD) + MOD) % MOD;
      window = (dropped * BASE + code(text[i + m])) % MOD; // @step roll
    }
  }
  return [hits.length, collisions, ...hits];
}
