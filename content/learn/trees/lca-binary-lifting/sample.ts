function build(parents: number[]): [number[][], number[]] {
  const n = parents.length + 1;
  const levels = Math.max(1, n.toString(2).length);
  const up = Array.from({ length: levels }, () => new Array<number>(n).fill(0));
  const depth = new Array<number>(n).fill(0);
  for (let v = 1; v < n; v++) {
    up[0][v] = parents[v - 1];
    depth[v] = depth[parents[v - 1]] + 1;
  }
  for (let k = 1; k < levels; k++) {
    for (let v = 0; v < n; v++) {
      up[k][v] = up[k - 1][up[k - 1][v]]; // @step table
    }
  }
  return [up, depth];
}

function lca(up: number[][], depth: number[], a: number, b: number): number {
  if (depth[a] < depth[b]) [a, b] = [b, a];
  const diff = depth[a] - depth[b];
  for (let k = 0; k < up.length; k++) {
    if ((diff >> k) & 1) {
      a = up[k][a]; // @step lift
    }
  }
  if (a === b) {
    return a; // @step same
  }
  for (let k = up.length - 1; k >= 0; k--) {
    if (up[k][a] !== up[k][b]) {
      [a, b] = [up[k][a], up[k][b]]; // @step jump
    }
  }
  return up[0][a]; // @step answer
}

function run(parents: number[], pairs: number[][]): number[] {
  const [up, depth] = build(parents);
  return pairs.map(([a, b]) => lca(up, depth, a, b));
}
