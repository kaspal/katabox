export function biggestTank(boards: number[]): number {
  let i = 0;
  let j = boards.length - 1;
  let best = 0;
  while (i < j) {
    if (boards[i] <= boards[j]) {
      best = Math.max(best, (j - i) * boards[i]);
      i++;
    } else {
      best = Math.max(best, (j - i) * boards[j]);
      j--;
    }
  }
  return best;
}
