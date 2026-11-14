function subsets(values: number[]): number[] {
  const out: number[][] = [];
  const chosen: number[] = [];
  const grow = (start: number): void => {
    out.push([...chosen]); // @step record
    for (let k = start; k < values.length; k++) {
      chosen.push(values[k]); // @step choose
      grow(k + 1);
      chosen.pop(); // @step undo
    }
  };
  grow(0);
  const flat = [out.length];
  for (const subset of out) flat.push(subset.length, ...subset);
  return flat;
}
