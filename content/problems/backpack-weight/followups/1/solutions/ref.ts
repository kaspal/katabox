export function bestPackCopies(weights: number[], values: number[], counts: number[], capacity: number): number {
  const best = new Array<number>(capacity + 1).fill(0);
  for (let i = 0; i < weights.length; i++) {
    let c = counts[i];
    let k = 1;
    while (c > 0) {
      const take = Math.min(k, c);
      const bw = weights[i] * take;
      const bv = values[i] * take;
      for (let cap = capacity; cap >= bw; cap--) if (best[cap - bw] + bv > best[cap]) best[cap] = best[cap - bw] + bv;
      c -= take;
      k *= 2;
    }
  }
  return best[capacity];
}
