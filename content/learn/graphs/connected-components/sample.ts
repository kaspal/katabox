function components(n: number, adj: number[][][]): number[] {
  const label = new Array<number>(n).fill(-1);
  let count = 0;
  for (let s = 0; s < n; s++) {
    if (label[s] !== -1) continue;
    label[s] = count; // @step new
    const stack = [s];
    while (stack.length > 0) {
      const u = stack.pop()!;
      for (const [v] of adj[u]) {
        if (label[v] === -1) {
          label[v] = count; // @step fill
          stack.push(v);
        }
      }
    }
    count++;
  }
  return label;
}
