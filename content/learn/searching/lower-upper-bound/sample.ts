function lowerBound(a: number[], target: number): number {
  let lo = 0;
  let hi = a.length;
  while (lo < hi) {
    const mid = Math.floor((lo + hi) / 2);
    if (a[mid] < target) { // @step lower
      lo = mid + 1;
    } else {
      hi = mid;
    }
  }
  return lo; // @step lower-done
}

function upperBound(a: number[], target: number): number {
  let lo = 0;
  let hi = a.length;
  while (lo < hi) {
    const mid = Math.floor((lo + hi) / 2);
    if (a[mid] <= target) { // @step upper
      lo = mid + 1;
    } else {
      hi = mid;
    }
  }
  return lo; // @step upper-done
}
