function fibMemo(n: number, memo: Map<number, number>): number {
  if (memo.has(n)) {
    return memo.get(n)!; // @step hit
  }
  const result = n < 2 ? n : fibMemo(n - 1, memo) + fibMemo(n - 2, memo);
  memo.set(n, result); // @step store
  return result;
}

function fibTable(n: number): number[] {
  const table = new Array<number>(n + 1).fill(0);
  if (n > 0) table[1] = 1;
  for (let i = 2; i <= n; i++) {
    table[i] = table[i - 1] + table[i - 2]; // @step fill
  }
  return table;
}

function run(n: number): number[] {
  return [fibMemo(n, new Map()), ...fibTable(n)];
}
