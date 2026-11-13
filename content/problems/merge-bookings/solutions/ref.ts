export function mergeBookings(bookings: number[][]): number[][] {
  const sorted = [...bookings].sort((a, b) => a[0] - b[0] || a[1] - b[1]);
  const out: number[][] = [];
  for (const [s, e] of sorted) {
    const last = out[out.length - 1];
    if (last !== undefined && s <= last[1]) last[1] = Math.max(last[1], e);
    else out.push([s, e]);
  }
  return out;
}
