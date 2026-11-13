export function bestPack(weights: number[], values: number[], capacity: number): number {
  const best = new Array<number>(capacity + 1).fill(0);
  weights.forEach((w, i) => {
    const v = values[i];
    for (let c = capacity; c >= w; c--) {
      if (best[c - w] + v > best[c]) best[c] = best[c - w] + v;
    }
  });
  return best[capacity];
}
