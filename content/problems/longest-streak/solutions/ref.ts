export function longestRun(values: number[]): number {
  let best = 1;
  let run = 1;
  for (let i = 1; i < values.length; i++) {
    run = values[i] === values[i - 1] ? run + 1 : 1;
    best = Math.max(best, run);
  }
  return best;
}
