function chainDepths(a: number[]): number[] {
  const n = a.length;
  const depth: number[] = new Array(n).fill(-1);
  let frontier: number[] = [];
  for (let i = 0; i < n; i++) {
    if (a[i] === -1) {
      depth[i] = 0; // @step anchor
      frontier.push(i);
    }
  }
  while (frontier.length > 0) {
    const found: number[] = [];
    for (const boss of frontier) {
      for (let e = 0; e < n; e++) {
        if (a[e] === boss) {
          depth[e] = depth[boss] + 1; // @step join
          found.push(e);
        }
      }
    }
    frontier = found;
  }
  return depth;
}
