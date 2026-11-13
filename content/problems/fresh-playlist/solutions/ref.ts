export function freshStretch(plays: number[]): number {
  const last = new Map<number, number>();
  let start = 0;
  let best = 0;
  plays.forEach((x, end) => {
    const seen = last.get(x);
    if (seen !== undefined && seen >= start) start = seen + 1;
    last.set(x, end);
    best = Math.max(best, end - start + 1);
  });
  return best;
}
