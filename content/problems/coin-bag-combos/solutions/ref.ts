export function coinCombos(coins: number[], target: number): number[][] {
  const sorted = [...coins].sort((a, b) => a - b);
  const out: number[][] = [];
  const bag: number[] = [];
  const pick = (start: number, remaining: number): void => {
    if (remaining === 0) {
      out.push([...bag]);
      return;
    }
    for (let i = start; i < sorted.length && sorted[i] <= remaining; i++) {
      bag.push(sorted[i]);
      pick(i, remaining - sorted[i]);
      bag.pop();
    }
  };
  pick(0, target);
  return out;
}
