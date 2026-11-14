function lis(values: number[]): number[] {
  const n = values.length;
  const length = new Array<number>(n).fill(1);
  const prev = new Array<number>(n).fill(-1);
  for (let i = 0; i < n; i++) {
    for (let j = 0; j < i; j++) {
      if (values[j] < values[i] && length[j] + 1 > length[i]) {
        length[i] = length[j] + 1; // @step extend
        prev[i] = j;
      }
    }
  }
  let end = 0;
  for (let k = 1; k < n; k++) if (length[k] > length[end]) end = k;
  const chain: number[] = [];
  while (end !== -1) {
    chain.push(values[end]); // @step trace
    end = prev[end];
  }
  return [chain.length, ...chain.reverse()];
}
