export function teamTotals(boss: number[], sales: number[]): number[] {
  const children: number[][] = boss.map(() => []);
  let owner = 0;
  boss.forEach((b, e) => {
    if (b < 0) owner = e;
    else children[b].push(e);
  });
  const order = [owner];
  for (let k = 0; k < order.length; k++) for (const c of children[order[k]]) order.push(c);
  const total = [...sales];
  for (let k = order.length - 1; k >= 0; k--) {
    const e = order[k];
    if (boss[e] >= 0) total[boss[e]] += total[e];
  }
  return total;
}
