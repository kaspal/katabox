const M = 7;

function build(keys: number[]): number[][] {
  const table: number[][] = Array.from({ length: M }, () => []);
  for (const key of keys) {
    const b = key % M; // @step hash
    if (table[b].includes(key)) { // @step scan
      continue;
    }
    table[b].push(key); // @step insert
  }
  return table;
}

function run(keys: number[]): number[] {
  const out: number[] = [];
  for (const bucket of build(keys)) out.push(bucket.length, ...bucket);
  return out;
}
