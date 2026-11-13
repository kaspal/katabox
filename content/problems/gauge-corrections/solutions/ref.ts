export function gaugeReadings(readings: number[], ops: string[], args: number[][]): number[] {
  const n = readings.length;
  const tree = new Array<number>(n + 1).fill(0);
  const add = (i: number, delta: number) => {
    for (let k = i + 1; k <= n; k += k & -k) tree[k] += delta;
  };
  const prefix = (i: number) => {
    let total = 0;
    for (let k = i; k > 0; k -= k & -k) total += tree[k];
    return total;
  };
  const out: number[] = [];
  ops.forEach((op, j) => {
    const a = args[j];
    if (op === "add") {
      add(a[0], a[2]);
      if (a[1] + 1 < n) add(a[1] + 1, -a[2]);
    } else {
      out.push(readings[a[0]] + prefix(a[0] + 1));
    }
  });
  return out;
}
