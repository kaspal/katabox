const P = 1_000_003;

function power(b: number, e: number): number {
  let r = 1;
  b %= P;
  while (e > 0) {
    if (e % 2 === 1) r = (r * b) % P;
    b = (b * b) % P;
    e = Math.floor(e / 2);
  }
  return r;
}

export function teamPicks(queries: number[][]): number[] {
  const top = queries.reduce((m, q) => Math.max(m, q[0]), 0);
  const fact = new Array<number>(top + 1).fill(1);
  for (let i = 1; i <= top; i++) fact[i] = (fact[i - 1] * i) % P;
  const inv = new Array<number>(top + 1).fill(1);
  inv[top] = power(fact[top], P - 2);
  for (let i = top; i > 0; i--) inv[i - 1] = (inv[i] * i) % P;
  return queries.map(([n, k]) => (k > n ? 0 : (((fact[n] * inv[k]) % P) * inv[n - k]) % P));
}
