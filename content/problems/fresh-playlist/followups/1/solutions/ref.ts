export function freshInRange(plays: number[], requests: number[][]): number[] {
  const n = plays.length;
  const start = new Array<number>(n).fill(0);
  const last = new Map<number, number>();
  let left = 0;
  for (let r = 0; r < n; r++) {
    const prev = last.get(plays[r]);
    if (prev !== undefined && prev >= left) left = prev + 1;
    last.set(plays[r], r);
    start[r] = left;
  }
  const table: number[][] = [start.map((s, r) => r - s + 1)];
  for (let j = 1; 1 << j <= n; j++) {
    const prev = table[j - 1];
    const half = 1 << (j - 1);
    const row = new Array<number>(n - (1 << j) + 1);
    for (let i = 0; i < row.length; i++) row[i] = Math.max(prev[i], prev[i + half]);
    table.push(row);
  }
  return requests.map(([a, b]) => {
    let lo = a;
    let hi = b + 1;
    while (lo < hi) {
      const mid = (lo + hi) >> 1;
      if (start[mid] >= a) hi = mid;
      else lo = mid + 1;
    }
    let best = lo - a;
    if (lo <= b) {
      const k = 31 - Math.clz32(b - lo + 1);
      best = Math.max(best, table[k][lo], table[k][b - (1 << k) + 1]);
    }
    return best;
  });
}
