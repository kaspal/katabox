function build(tree: number[], a: number[], node: number, lo: number, hi: number): void {
  if (lo === hi) {
    tree[node] = a[lo]; // @step leaf
    return;
  }
  const mid = Math.floor((lo + hi) / 2);
  build(tree, a, 2 * node, lo, mid);
  build(tree, a, 2 * node + 1, mid + 1, hi);
  tree[node] = tree[2 * node] + tree[2 * node + 1]; // @step merge
}

function query(tree: number[], node: number, lo: number, hi: number, left: number, right: number): number {
  if (right < lo || hi < left) {
    return 0; // @step outside
  }
  if (left <= lo && hi <= right) {
    return tree[node]; // @step inside
  }
  const mid = Math.floor((lo + hi) / 2);
  return query(tree, 2 * node, lo, mid, left, right) + query(tree, 2 * node + 1, mid + 1, hi, left, right);
}

function update(tree: number[], node: number, lo: number, hi: number, i: number, value: number): void {
  if (lo === hi) {
    tree[node] = value; // @step set
    return;
  }
  const mid = Math.floor((lo + hi) / 2);
  if (i <= mid) update(tree, 2 * node, lo, mid, i, value);
  else update(tree, 2 * node + 1, mid + 1, hi, i, value);
  tree[node] = tree[2 * node] + tree[2 * node + 1]; // @step recompute
}

function run(a: number[], ops: (string | number)[][]): number[] {
  const n = a.length;
  const tree = new Array<number>(4 * n).fill(0);
  build(tree, a, 1, 0, n - 1);
  const out: number[] = [];
  for (const op of ops) {
    if (op[0] === "sum") out.push(query(tree, 1, 0, n - 1, op[1] as number, op[2] as number));
    else update(tree, 1, 0, n - 1, op[1] as number, op[2] as number);
  }
  return out;
}
