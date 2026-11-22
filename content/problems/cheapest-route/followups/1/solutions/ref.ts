export function cheapestWithin(n: number, roads: number[][], start: number, goal: number, k: number): number {
  let best = new Array<number>(n).fill(Infinity);
  best[start] = 0;
  for (let r = 0; r < k; r++) {
    const nxt = best.slice();
    let changed = false;
    for (const [a, b, t] of roads) {
      if (best[a] + t < nxt[b]) {
        nxt[b] = best[a] + t;
        changed = true;
      }
      if (best[b] + t < nxt[a]) {
        nxt[a] = best[b] + t;
        changed = true;
      }
    }
    best = nxt;
    if (!changed) break;
  }
  return best[goal] === Infinity ? -1 : best[goal];
}
