export function rowLedger(stock: number[], ops: string[], args: number[][]): number[] {
  const n = stock.length;
  const b1 = new Array<number>(n + 1).fill(0);
  const b2 = new Array<number>(n + 1).fill(0);
  const add = (t: number[], i: number, v: number) => {
    for (; i <= n; i += i & -i) t[i] += v;
  };
  const query = (t: number[], i: number) => {
    let s = 0;
    for (; i > 0; i -= i & -i) s += t[i];
    return s;
  };
  const rangeAdd = (a: number, b: number, v: number) => {
    add(b1, a, v);
    add(b1, b + 1, -v);
    add(b2, a, v * (a - 1));
    add(b2, b + 1, -v * b);
  };
  const prefix = (i: number) => query(b1, i) * i - query(b2, i);
  for (let i = 0; i < n; i++) rangeAdd(i + 1, i + 1, stock[i]);
  const out: number[] = [];
  for (let k = 0; k < ops.length; k++) {
    const a = args[k];
    if (ops[k] === "add") rangeAdd(a[0] + 1, a[1] + 1, a[2]);
    else out.push(prefix(a[1] + 1) - prefix(a[0]));
  }
  return out;
}
