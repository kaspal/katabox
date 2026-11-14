function twoColour(n: number, adj: number[][][]): number[] {
  const side = new Array<number>(n).fill(-1);
  for (let s = 0; s < n; s++) {
    if (side[s] !== -1) continue;
    side[s] = 0; // @step start
    const queue = [s];
    let head = 0;
    while (head < queue.length) {
      const u = queue[head++];
      for (const [v] of adj[u]) {
        if (side[v] === -1) {
          side[v] = 1 - side[u]; // @step colour
          queue.push(v);
        } else if (side[v] === side[u]) {
          return [-1]; // @step clash
        }
      }
    }
  }
  return side;
}
