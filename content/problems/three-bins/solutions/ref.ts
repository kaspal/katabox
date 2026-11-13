export function sortBins(items: number[]): number[] {
  let low = 0;
  let mid = 0;
  let high = items.length - 1;
  while (mid <= high) {
    if (items[mid] === 0) {
      [items[low], items[mid]] = [items[mid], items[low]];
      low++;
      mid++;
    } else if (items[mid] === 1) {
      mid++;
    } else {
      [items[mid], items[high]] = [items[high], items[mid]];
      high--;
    }
  }
  return items;
}
