function bubbleSort(a: number[]): number[] {
  const n = a.length;
  for (let end = n - 1; end > 0; end--) { // @step pass
    let swapped = false;
    for (let j = 0; j < end; j++) {
      if (a[j] > a[j + 1]) { // @step compare
        [a[j], a[j + 1]] = [a[j + 1], a[j]]; // @step swap
        swapped = true;
      }
    }
    if (!swapped) break; // @step check
  }
  return a;
}
