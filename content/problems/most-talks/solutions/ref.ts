export function mostTalks(talks: number[][]): number {
  const sorted = [...talks].sort((a, b) => a[1] - b[1]);
  let count = 0;
  let free = -1;
  for (const [start, end] of sorted) {
    if (start >= free) {
      count++;
      free = end;
    }
  }
  return count;
}
