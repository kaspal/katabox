export function stockLedger(stock: number[], ops: string[], args: number[][]): number[] {
  const n = stock.length;
  const tree = new Array<number>(n + 1).fill(0);
  const add = (i: number, delta: number) => {
    for (let k = i + 1; k <= n; k += k & -k) tree[k] += delta;
  };
  const prefix = (i: number) => {
    let total = 0;
    for (let k = i; k > 0; k -= k & -k) total += tree[k];
    return total;
  };
  const current = [...stock];
  stock.forEach((v, i) => add(i, v));
  const out: number[] = [];
  ops.forEach((op, j) => {
    const [x, y] = args[j];
    if (op === "set") {
      add(x, y - current[x]);
      current[x] = y;
    } else {
      out.push(prefix(y + 1) - prefix(x));
    }
  });
  return out;
}
