export function kinshipDistances(parent: number[], pairs: number[][]): number[] {
  const n = parent.length;
  const children: number[][] = Array.from({ length: n }, () => []);
  let root = 0;
  parent.forEach((p, i) => (p === -1 ? (root = i) : children[p].push(i)));
  const depth = new Array<number>(n).fill(0);
  const order = [root];
  for (let h = 0; h < order.length; h++)
    for (const v of children[order[h]]) {
      depth[v] = depth[order[h]] + 1;
      order.push(v);
    }
  let levels = 1;
  while (1 << levels < n) levels++;
  const up: number[][] = [parent.map((p) => (p === -1 ? root : p))];
  for (let k = 1; k < levels; k++) {
    const prev = up[k - 1];
    up.push(prev.map((p) => prev[p]));
  }
  return pairs.map(([a, b]) => {
    let x = depth[a] >= depth[b] ? a : b;
    let y = x === a ? b : a;
    let steps = depth[x] - depth[y];
    for (let k = 0; steps > 0; k++, steps >>= 1) if (steps & 1) x = up[k][x];
    if (x !== y) {
      for (let k = levels - 1; k >= 0; k--)
        if (up[k][x] !== up[k][y]) {
          x = up[k][x];
          y = up[k][y];
        }
      x = up[0][x];
    }
    return depth[a] + depth[b] - 2 * depth[x];
  });
}
