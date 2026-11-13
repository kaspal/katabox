export function coldSnaps(temps: number[], ops: string[], args: number[][]): number[] {
  const n = temps.length;
  let size = 1;
  while (size < n) size *= 2;
  const mn = new Array<number>(2 * size).fill(Infinity);
  const lazy = new Array<number>(2 * size).fill(0);
  for (let i = 0; i < n; i++) mn[size + i] = temps[i];
  for (let v = size - 1; v > 0; v--) mn[v] = Math.min(mn[2 * v], mn[2 * v + 1]);
  const push = (v: number) => {
    if (lazy[v] !== 0) {
      for (const c of [2 * v, 2 * v + 1]) {
        mn[c] += lazy[v];
        lazy[c] += lazy[v];
      }
      lazy[v] = 0;
    }
  };
  const shift = (v: number, lo: number, hi: number, a: number, b: number, d: number): void => {
    if (b < lo || hi < a) return;
    if (a <= lo && hi <= b) {
      mn[v] += d;
      lazy[v] += d;
      return;
    }
    push(v);
    const mid = (lo + hi) >> 1;
    shift(2 * v, lo, mid, a, b, d);
    shift(2 * v + 1, mid + 1, hi, a, b, d);
    mn[v] = Math.min(mn[2 * v], mn[2 * v + 1]);
  };
  const coldest = (v: number, lo: number, hi: number, a: number, b: number): number => {
    if (b < lo || hi < a) return Infinity;
    if (a <= lo && hi <= b) return mn[v];
    push(v);
    const mid = (lo + hi) >> 1;
    return Math.min(coldest(2 * v, lo, mid, a, b), coldest(2 * v + 1, mid + 1, hi, a, b));
  };
  const out: number[] = [];
  ops.forEach((op, j) => {
    const a = args[j];
    if (op === "shift") shift(1, 0, size - 1, a[0], a[1], a[2]);
    else out.push(coldest(1, 0, size - 1, a[0], a[1]));
  });
  return out;
}
