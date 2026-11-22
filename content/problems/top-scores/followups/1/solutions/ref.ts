export function kthAfterEach(scores: number[], k: number): number[] {
  // Scores are 0..300: count them and track the k-th best by walking the counts.
  const count = new Array<number>(301).fill(0);
  const out: number[] = [];
  let kth = -1; // the current k-th best once there are k scores
  let above = 0; // scores strictly above kth
  let seen = 0;
  for (const s of scores) {
    count[s]++;
    seen++;
    if (seen < k) {
      out.push(-1);
      continue;
    }
    if (seen === k) {
      // The smallest score is the k-th best.
      kth = 0;
      while (count[kth] === 0) kth++;
      above = k - count[kth];
    } else if (s > kth) {
      above++;
      // Too many above: move the k-th best up to the next score that is present.
      while (above >= k) {
        kth++;
        while (count[kth] === 0) kth++;
        above -= count[kth];
      }
    }
    out.push(kth);
  }
  return out;
}
