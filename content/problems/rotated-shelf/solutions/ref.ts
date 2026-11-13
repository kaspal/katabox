export function findLabel(shelf: number[], label: number): number {
  let lo = 0;
  let hi = shelf.length - 1;
  while (lo <= hi) {
    const m = (lo + hi) >> 1;
    if (shelf[m] === label) return m;
    if (shelf[lo] <= shelf[m]) {
      if (shelf[lo] <= label && label < shelf[m]) hi = m - 1;
      else lo = m + 1;
    } else if (shelf[m] < label && label <= shelf[hi]) {
      lo = m + 1;
    } else {
      hi = m - 1;
    }
  }
  return -1;
}
