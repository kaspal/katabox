function kadane(values: number[]): number[] {
  let best = values[0];
  let bestStart = 0;
  let bestEnd = 0;
  let current = 0;
  let start = 0;
  values.forEach((value, i) => {
    if (current <= 0) {
      current = value; // @step restart
      start = i;
    } else {
      current += value; // @step extend
    }
    if (current > best) {
      best = current; // @step record
      bestStart = start;
      bestEnd = i;
    }
  });
  return [best, bestStart, bestEnd];
}
