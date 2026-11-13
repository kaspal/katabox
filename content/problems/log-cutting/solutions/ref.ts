export function cutCost(length: number, marks: number[]): number {
  const p = [0, ...[...marks].sort((a, b) => a - b), length];
  const m = p.length;
  const cost: number[][] = Array.from({ length: m }, () => new Array<number>(m).fill(0));
  for (let gap = 2; gap < m; gap++) {
    for (let i = 0; i + gap < m; i++) {
      const j = i + gap;
      let best = Infinity;
      for (let k = i + 1; k < j; k++) best = Math.min(best, cost[i][k] + cost[k][j]);
      cost[i][j] = p[j] - p[i] + best;
    }
  }
  return cost[0][m - 1];
}
