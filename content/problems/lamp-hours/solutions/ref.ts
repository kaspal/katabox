export function litTime(lamps: number[][]): number {
  const ordered = [...lamps].sort((a, b) => a[0] - b[0] || a[1] - b[1]);
  let [s, e] = ordered[0];
  let total = 0;
  for (let i = 1; i < ordered.length; i++) {
    const [start, end] = ordered[i];
    if (start <= e) {
      e = Math.max(e, end);
    } else {
      total += e - s;
      s = start;
      e = end;
    }
  }
  return total + e - s;
}
