function merge(intervals: number[][]): number[] {
  const merged: number[][] = [];
  const sorted = [...intervals].sort((a, b) => a[0] - b[0] || a[1] - b[1]);
  for (const [start, end] of sorted) {
    if (merged.length > 0 && start <= merged[merged.length - 1][1]) {
      merged[merged.length - 1][1] = Math.max(merged[merged.length - 1][1], end); // @step extend
    } else {
      merged.push([start, end]); // @step open
    }
  }
  return merged.flat();
}
