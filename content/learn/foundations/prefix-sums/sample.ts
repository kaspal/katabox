function prefixSums(a: number[]): number[] {
  const p = new Array<number>(a.length + 1).fill(0);
  for (let i = 0; i < a.length; i++) {
    p[i + 1] = p[i] + a[i]; // @step add
  }
  return p;
}

function rangeSum(p: number[], lo: number, hi: number): number {
  return p[hi + 1] - p[lo]; // @step query
}
