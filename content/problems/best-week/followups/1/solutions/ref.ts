export function bestRun(profits: number[], k: number): number {
  const n = profits.length;
  const prefix = new Array<number>(n + 1).fill(0);
  for (let i = 0; i < n; i++) prefix[i + 1] = prefix[i] + profits[i];
  const lows = new Array<number>(n + 1);
  let head = 0;
  let tail = 0;
  let best = -Infinity;
  for (let j = 1; j <= n; j++) {
    const i = j - 1;
    while (tail > head && prefix[lows[tail - 1]] >= prefix[i]) tail--;
    lows[tail++] = i;
    if (lows[head] < j - k) head++;
    best = Math.max(best, prefix[j] - prefix[lows[head]]);
  }
  return best;
}
