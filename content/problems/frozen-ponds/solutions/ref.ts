export function pondSizes(ice: string[]): number[] {
  const h = ice.length;
  const w = ice[0].length;
  const parent = new Int32Array(h * w);
  const size = new Int32Array(h * w).fill(1);
  for (let i = 0; i < h * w; i++) parent[i] = i;
  const find = (x: number): number => {
    while (parent[x] !== x) {
      parent[x] = parent[parent[x]];
      x = parent[x];
    }
    return x;
  };
  const union = (a: number, b: number) => {
    a = find(a);
    b = find(b);
    if (a === b) return;
    if (size[a] < size[b]) [a, b] = [b, a];
    parent[b] = a;
    size[a] += size[b];
  };
  for (let r = 0; r < h; r++) {
    for (let c = 0; c < w; c++) {
      if (ice[r][c] !== "#") continue;
      if (c + 1 < w && ice[r][c + 1] === "#") union(r * w + c, r * w + c + 1);
      if (r + 1 < h && ice[r + 1][c] === "#") union(r * w + c, (r + 1) * w + c);
    }
  }
  const sizes: number[] = [];
  for (let r = 0; r < h; r++) {
    for (let c = 0; c < w; c++) {
      const i = r * w + c;
      if (ice[r][c] === "#" && find(i) === i) sizes.push(size[i]);
    }
  }
  return sizes.sort((a, b) => b - a);
}
