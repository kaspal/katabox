export function longestRun(values: number[]): number {
  let best = 0;
  let run = 1;
  for (let i = 1; i < values.length; i++) {
    if (values[i] === values[i - 1]) run++;
    else {
      best = Math.max(best, run);
      run = 1;
    }
  }
  return best;
}
