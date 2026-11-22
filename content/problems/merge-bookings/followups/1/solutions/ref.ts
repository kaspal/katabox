function merge(bookings: number[][]): number[][] {
  const sorted = [...bookings].sort((x, y) => x[0] - y[0] || x[1] - y[1]);
  const out: number[][] = [];
  for (const [s, e] of sorted) {
    const last = out[out.length - 1];
    if (last && s <= last[1]) last[1] = Math.max(last[1], e);
    else out.push([s, e]);
  }
  return out;
}

export function bothBusy(roomA: number[][], roomB: number[][]): number[][] {
  const a = merge(roomA);
  const b = merge(roomB);
  const out: number[][] = [];
  let i = 0;
  let j = 0;
  while (i < a.length && j < b.length) {
    const s = Math.max(a[i][0], b[j][0]);
    const e = Math.min(a[i][1], b[j][1]);
    if (s <= e) out.push([s, e]);
    if (a[i][1] < b[j][1]) i++;
    else j++;
  }
  return out;
}
