export function commonBoss(boss: number[], pairs: number[][]): number[] {
  const n = boss.length;
  const children: number[][] = boss.map(() => []);
  let president = 0;
  boss.forEach((b, v) => {
    if (b < 0) president = v;
    else children[b].push(v);
  });
  const depth = new Int32Array(n);
  const order = [president];
  for (let h = 0; h < order.length; h++) {
    const v = order[h];
    for (const c of children[v]) {
      depth[c] = depth[v] + 1;
      order.push(c);
    }
  }
  let levels = 1;
  while (1 << levels < n) levels++;
  const up: Int32Array[] = [Int32Array.from(boss, (b) => (b < 0 ? president : b))];
  for (let k = 1; k < levels; k++) {
    const prev = up[k - 1];
    const cur = new Int32Array(n);
    for (let v = 0; v < n; v++) cur[v] = prev[prev[v]];
    up.push(cur);
  }
  return pairs.map(([x, y]) => {
    let a = x;
    let b = y;
    if (depth[a] < depth[b]) [a, b] = [b, a];
    let d = depth[a] - depth[b];
    for (let k = 0; d > 0; k++, d >>= 1) if (d & 1) a = up[k][a];
    if (a === b) return a;
    for (let k = levels - 1; k >= 0; k--) {
      if (up[k][a] !== up[k][b]) {
        a = up[k][a];
        b = up[k][b];
      }
    }
    return up[0][a];
  });
}
