export function freshWithEncores(plays: number[]): number {
  const count = new Map<number, number>();
  let best = 0;
  let start = 0;
  for (let end = 0; end < plays.length; end++) {
    const song = plays[end];
    count.set(song, (count.get(song) ?? 0) + 1);
    while (count.get(song)! > 2) {
      count.set(plays[start], count.get(plays[start])! - 1);
      start++;
    }
    best = Math.max(best, end - start + 1);
  }
  return best;
}
