function exponentialSearch(a: number[], target: number): number {
  if (a[0] === target) return 0; // @step first
  let bound = 1;
  while (bound < a.length && a[bound] < target) { // @step double
    bound *= 2;
  }
  let lo = Math.floor(bound / 2); // @step range
  let hi = Math.min(bound, a.length - 1);
  while (lo <= hi) {
    const mid = Math.floor((lo + hi) / 2); // @step mid
    if (a[mid] === target) {
      return mid; // @step found
    }
    if (a[mid] < target) { // @step compare
      lo = mid + 1;
    } else {
      hi = mid - 1;
    }
  }
  return -1; // @step missing
}
