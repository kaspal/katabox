const P = 1_000_003;

export function blockedWalks(rows: number, cols: number, blocked: number[][]): number {
  const top = rows + cols;
  const fact = new Array<number>(top + 1).fill(1);
  for (let i = 1; i <= top; i++) fact[i] = (fact[i - 1] * i) % P;
  const inv = new Array<number>(top + 1).fill(1);
  let r = 1;
  let b = fact[top];
  for (let e = P - 2; e > 0; e = Math.floor(e / 2)) {
    if (e % 2 === 1) r = (r * b) % P;
    b = (b * b) % P;
  }
  inv[top] = r;
  for (let i = top; i > 0; i--) inv[i - 1] = (inv[i] * i) % P;
  const paths = (dr: number, dc: number) => (((fact[dr + dc] * inv[dr]) % P) * inv[dc]) % P;

  const points = [...blocked].sort((x, y) => x[0] - y[0] || x[1] - y[1]);
  points.push([rows, cols]);
  const good: number[] = [];
  for (let i = 0; i < points.length; i++) {
    const [pr, pc] = points[i];
    let ways = paths(pr, pc);
    for (let j = 0; j < i; j++) {
      const [qr, qc] = points[j];
      if (qr <= pr && qc <= pc) ways = (ways - ((good[j] * paths(pr - qr, pc - qc)) % P) + P) % P;
    }
    good.push(ways);
  }
  return good[good.length - 1];
}
