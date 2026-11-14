const INF = 999;

function coinChange(amount: number, coins: number[]): number[] {
  const fewest = [0, ...new Array<number>(amount).fill(INF)];
  for (let total = 1; total <= amount; total++) {
    for (const coin of coins) {
      if (coin <= total && fewest[total - coin] + 1 < fewest[total]) {
        fewest[total] = fewest[total - coin] + 1; // @step improve
      }
    }
  }
  return [fewest[amount] < INF ? fewest[amount] : -1];
}
