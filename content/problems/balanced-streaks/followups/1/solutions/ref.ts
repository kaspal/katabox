export function longestBalanced(results: string): number {
  const first = new Map<number, number>([[0, -1]]);
  let score = 0;
  let best = 0;
  for (let i = 0; i < results.length; i++) {
    const c = results[i];
    if (c === "W") score++;
    else if (c === "L") score--;
    const f = first.get(score);
    if (f !== undefined) best = Math.max(best, i - f);
    else first.set(score, i);
  }
  return best;
}
