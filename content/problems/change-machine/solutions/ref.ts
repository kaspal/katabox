export function fewestCoins(coins: number[], amount: number): number {
  const inf = amount + 1;
  const best = new Int32Array(amount + 1).fill(inf);
  best[0] = 0;
  for (let x = 1; x <= amount; x++) {
    let b = inf;
    for (const c of coins) {
      if (c <= x && best[x - c] + 1 < b) b = best[x - c] + 1;
    }
    best[x] = b;
  }
  return best[amount] <= amount ? best[amount] : -1;
}
