function permutations(values: number[]): number[] {
  const n = values.length;
  const used = new Array<boolean>(n).fill(false);
  const chosen: number[] = [];
  const out: number[] = [];
  const place = (): void => {
    if (chosen.length === n) {
      out.push(...chosen); // @step record
      return;
    }
    for (let k = 0; k < n; k++) {
      if (!used[k]) {
        used[k] = true;
        chosen.push(values[k]); // @step choose
        place();
        chosen.pop(); // @step undo
        used[k] = false;
      }
    }
  };
  place();
  return [out.length / n, ...out];
}
