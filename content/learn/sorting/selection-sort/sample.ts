function selectionSort(a: number[]): void {
  const n = a.length;
  for (let i = 0; i < n - 1; i++) {
    let smallest = i; // @step start
    for (let j = i + 1; j < n; j++) {
      if (a[j] < a[smallest]) { // @step compare
        smallest = j; // @step new-min
      }
    }
    [a[i], a[smallest]] = [a[smallest], a[i]]; // @step swap
  }
}
