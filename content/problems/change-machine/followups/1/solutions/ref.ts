const MOD = 1000000007;

export function countWays(coins: number[], amount: number): number {
  const ways = new Array<number>(amount + 1).fill(0);
  ways[0] = 1;
  for (const c of coins) for (let a = c; a <= amount; a++) ways[a] = (ways[a] + ways[a - c]) % MOD;
  return ways[amount];
}
