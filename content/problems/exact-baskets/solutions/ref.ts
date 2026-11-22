export function exactBaskets(weights: number[], k: number, target: number): number[][] {
  const items = [...weights].sort((a, b) => a - b);
  const out: number[][] = [];
  const pick: number[] = [];
  const go = (i: number, total: number): void => {
    if (pick.length === k) {
      if (total === target) out.push([...pick]);
      return;
    }
    for (let j = i; j < items.length; j++) {
      pick.push(items[j]);
      go(j + 1, total + items[j]);
      pick.pop();
    }
  };
  go(0, 0);
  return out;
}
