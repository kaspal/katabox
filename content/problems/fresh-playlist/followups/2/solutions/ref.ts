export function longestRotation(plays: number[], k: number): number {
  const count = new Map<number, number>();
  let best = 0;
  let start = 0;
  for (let end = 0; end < plays.length; end++) {
    const song = plays[end];
    count.set(song, (count.get(song) ?? 0) + 1);
    while (count.size > k) {
      const s = plays[start++];
      const c = count.get(s)! - 1;
      if (c === 0) count.delete(s);
      else count.set(s, c);
    }
    best = Math.max(best, end - start + 1);
  }
  return best;
}
