function add(tree: number[], i: number, value: number): void {
  for (i += 1; i < tree.length; i += i & -i) {
    tree[i] += value; // @step add
  }
}

function prefix(tree: number[], i: number): number {
  let total = 0;
  for (i += 1; i > 0; i -= i & -i) {
    total += tree[i]; // @step read
  }
  return total;
}

function run(n: number, ops: (string | number)[][]): number[] {
  const tree = new Array<number>(n + 1).fill(0);
  const out: number[] = [];
  for (const op of ops) {
    if (op[0] === "add") add(tree, op[1] as number, op[2] as number);
    else out.push(prefix(tree, op[1] as number));
  }
  return out;
}
