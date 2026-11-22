function select(intervals: number[][]): number[] {
  const order = intervals
    .map((_, k) => k)
    .sort(
      (x, y) =>
        intervals[x][1] - intervals[y][1] ||
        intervals[x][0] - intervals[y][0] ||
        x - y,
    );
  const chosen: number[] = [];
  let freeAt: number | null = null;
  for (const k of order) {
    const [start, end] = intervals[k];
    if (freeAt === null || start >= freeAt) {
      chosen.push(k); // @step take
      freeAt = end;
    } else {
      continue; // @step clash
    }
  }
  return [chosen.length, ...chosen.sort((a, b) => a - b)];
}
