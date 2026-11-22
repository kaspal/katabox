export function kthInWindow(scores: number[], w: number, k: number): number[] {
  const count = new Array<number>(301).fill(0);
  const out: number[] = [];
  for (let i = 0; i < scores.length; i++) {
    count[scores[i]]++;
    if (i >= w) count[scores[i - w]]--;
    if (i >= w - 1) {
      let left = k;
      let v = 300;
      while (left > count[v]) {
        left -= count[v];
        v--;
      }
      out.push(v);
    }
  }
  return out;
}
