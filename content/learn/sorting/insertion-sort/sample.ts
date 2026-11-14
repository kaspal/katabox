function insertionSort(a: number[]): number[] {
  for (let i = 1; i < a.length; i++) {
    const key = a[i]; // @step take
    let j = i - 1;
    while (j >= 0 && a[j] > key) { // @step compare
      a[j + 1] = a[j]; // @step shift
      j--;
    }
    a[j + 1] = key; // @step insert
  }
  return a;
}
