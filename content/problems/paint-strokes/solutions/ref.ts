export function paintedLength(strokes: number[][]): number {
  const s = [...strokes].sort((a, b) => a[0] - b[0]);
  let total = 0;
  let [runStart, runEnd] = s[0];
  for (let i = 1; i < s.length; i++) {
    const [start, end] = s[i];
    if (start <= runEnd) {
      if (end > runEnd) runEnd = end;
    } else {
      total += runEnd - runStart;
      runStart = start;
      runEnd = end;
    }
  }
  return total + runEnd - runStart;
}
