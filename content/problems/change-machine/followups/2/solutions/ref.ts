export function fewestTokens(tokens: number[], amount: number): number {
  const inf = tokens.length + 1;
  const best = new Array<number>(amount + 1).fill(inf);
  best[0] = 0;
  for (const t of tokens)
    for (let a = amount; a >= t; a--) if (best[a - t] + 1 < best[a]) best[a] = best[a - t] + 1;
  return best[amount] < inf ? best[amount] : -1;
}
