function daysNeeded(weights: number[], capacity: number): number {
  let days = 1;
  let load = 0;
  for (const w of weights) {
    if (load + w > capacity) {
      days++;
      load = 0;
    }
    load += w;
  }
  return days;
}

function minCapacity(weights: number[], days: number): number {
  let lo = Math.max(...weights);
  let hi = weights.reduce((s, w) => s + w, 0);
  while (lo < hi) {
    const mid = Math.floor((lo + hi) / 2); // @step try
    if (daysNeeded(weights, mid) <= days) { // @step check
      hi = mid;
    } else {
      lo = mid + 1;
    }
  }
  return lo; // @step answer
}
