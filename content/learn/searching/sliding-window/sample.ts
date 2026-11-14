function longestWithin(a: number[], limit: number): number[] {
  let best = 0;
  let bestStart = 0;
  let total = 0;
  let lo = 0;
  for (let hi = 0; hi < a.length; hi++) {
    total += a[hi]; // @step grow
    while (total > limit) {
      total -= a[lo]; // @step shrink
      lo++;
    }
    if (hi - lo + 1 > best) { // @step check
      best = hi - lo + 1;
      bestStart = lo;
    }
  }
  return [best, bestStart];
}
