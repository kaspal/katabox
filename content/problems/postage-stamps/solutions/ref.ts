export function exactPostage(values: number[], target: number): number[] {
  const n = values.length;
  const suffix = new Array<number>(n + 1).fill(0);
  for (let i = n - 1; i >= 0; i--) suffix[i] = suffix[i + 1] + values[i];
  let best: number[] | null = null;
  const chosen: number[] = [];
  const search = (i: number, missing: number): void => {
    if (missing === 0) {
      if (best === null || chosen.length < best.length) best = [...chosen];
      return;
    }
    if (i === n || missing > suffix[i]) return;
    if (best !== null && chosen.length + 1 >= best.length) return;
    if (values[i] <= missing) {
      chosen.push(i);
      search(i + 1, missing - values[i]);
      chosen.pop();
    }
    search(i + 1, missing);
  };
  search(0, target);
  return best ?? [];
}
