export function tollRefunds(n: number, roads: number[][], start: number): number[] {
  const cost = new Array<number>(n).fill(Infinity);
  cost[start] = 0;
  for (let round = 0; round < n - 1; round++) {
    let changed = false;
    for (const [a, b, c] of roads) {
      if (cost[a] !== Infinity && cost[a] + c < cost[b]) {
        cost[b] = cost[a] + c;
        changed = true;
      }
    }
    if (!changed) break;
  }
  for (const [a, b, c] of roads) {
    if (cost[a] !== Infinity && cost[a] + c < cost[b]) return [];
  }
  return cost.map((x) => (x === Infinity ? 1_000_000_000 : x));
}
