function binarySearch(a: number[], target: number): number {
  let lo = 0;
  let hi = a.length - 1;
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
