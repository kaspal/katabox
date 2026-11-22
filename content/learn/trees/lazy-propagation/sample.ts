function apply(
  tree: number[],
  lazy: number[],
  node: number,
  lo: number,
  hi: number,
  value: number,
): void {
  tree[node] += value * (hi - lo + 1);
  lazy[node] += value;
}

function push(
  tree: number[],
  lazy: number[],
  node: number,
  lo: number,
  hi: number,
): void {
  if (lazy[node] !== 0) {
    const mid = Math.floor((lo + hi) / 2);
    apply(tree, lazy, 2 * node, lo, mid, lazy[node]); // @step push-down
    apply(tree, lazy, 2 * node + 1, mid + 1, hi, lazy[node]);
    lazy[node] = 0;
  }
}

function add(
  tree: number[],
  lazy: number[],
  node: number,
  lo: number,
  hi: number,
  left: number,
  right: number,
  value: number,
): void {
  if (right < lo || hi < left) return;
  if (left <= lo && hi <= right) {
    apply(tree, lazy, node, lo, hi, value); // @step tag
    return;
  }
  push(tree, lazy, node, lo, hi);
  const mid = Math.floor((lo + hi) / 2);
  add(tree, lazy, 2 * node, lo, mid, left, right, value);
  add(tree, lazy, 2 * node + 1, mid + 1, hi, left, right, value);
  tree[node] = tree[2 * node] + tree[2 * node + 1]; // @step recompute
}

function total(
  tree: number[],
  lazy: number[],
  node: number,
  lo: number,
  hi: number,
  left: number,
  right: number,
): number {
  if (right < lo || hi < left) return 0;
  if (left <= lo && hi <= right) {
    return tree[node]; // @step inside
  }
  push(tree, lazy, node, lo, hi);
  const mid = Math.floor((lo + hi) / 2);
  return (
    total(tree, lazy, 2 * node, lo, mid, left, right) +
    total(tree, lazy, 2 * node + 1, mid + 1, hi, left, right)
  );
}

function run(n: number, ops: (string | number)[][]): number[] {
  const tree = new Array<number>(4 * n).fill(0);
  const lazy = new Array<number>(4 * n).fill(0);
  const out: number[] = [];
  for (const op of ops) {
    const [kind, a, b, value] = op as [string, number, number, number];
    if (kind === "add") add(tree, lazy, 1, 0, n - 1, a, b, value);
    else out.push(total(tree, lazy, 1, 0, n - 1, a, b));
  }
  return out;
}
