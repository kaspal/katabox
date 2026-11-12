export function usersWithAll(users: number[], queries: number[]): number[] {
  let top = 1;
  for (const x of users) top = Math.max(top, x);
  for (const x of queries) top = Math.max(top, x);
  let bits = 0;
  while (1 << bits <= top) bits++;
  const size = 1 << bits;
  const count = new Int32Array(size);
  for (const u of users) count[u]++;
  for (let b = 0; b < bits; b++) {
    const bit = 1 << b;
    for (let m = 0; m < size; m++) {
      if (!(m & bit)) count[m] += count[m | bit];
    }
  }
  return queries.map((q) => count[q]);
}
