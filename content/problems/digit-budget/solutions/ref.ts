export function digitBudget(limit: number, total: number): number {
  const digits = String(limit).split("").map(Number);
  const n = digits.length;
  const memo = new Map<number, number>();
  const free = (pos: number, s: number): number => {
    if (s > total) return 0;
    if (pos === n) return s === total ? 1 : 0;
    const key = pos * 1000 + s;
    const known = memo.get(key);
    if (known !== undefined) return known;
    let count = 0;
    for (let x = 0; x <= 9; x++) count += free(pos + 1, s + x);
    memo.set(key, count);
    return count;
  };
  let count = 0;
  let s = 0;
  digits.forEach((top, pos) => {
    for (let x = 0; x < top; x++) count += free(pos + 1, s + x);
    s += top;
  });
  return count + (s === total ? 1 : 0);
}
