function combinationSum(target: number, candidates: number[]): number[] {
  const sorted = [...candidates].sort((a, b) => a - b);
  const out: number[][] = [];
  const chosen: number[] = [];
  const search = (start: number, left: number): void => {
    if (left === 0) {
      out.push([...chosen]); // @step found
      return;
    }
    for (let k = start; k < sorted.length; k++) {
      if (sorted[k] > left) {
        break; // @step prune
      }
      chosen.push(sorted[k]); // @step choose
      search(k, left - sorted[k]);
      chosen.pop();
    }
  };
  search(0, target);
  const flat = [out.length];
  for (const combo of out) flat.push(combo.length, ...combo);
  return flat;
}
