function shellSort(a: number[]): void {
  let gap = Math.floor(a.length / 2);
  while (gap > 0) { // @step gap
    for (let i = gap; i < a.length; i++) {
      const key = a[i]; // @step take
      let j = i;
      while (j >= gap && a[j - gap] > key) { // @step compare
        a[j] = a[j - gap]; // @step shift
        j -= gap;
      }
      a[j] = key; // @step insert
    }
    gap = Math.floor(gap / 2);
  }
}
