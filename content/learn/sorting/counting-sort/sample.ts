function countingSort(a: number[]): void {
  const counts = new Array<number>(Math.max(...a) + 1).fill(0);
  for (const v of a) {
    counts[v]++; // @step count
  }
  let k = 0;
  for (let v = 0; v < counts.length; v++) {
    for (let c = 0; c < counts[v]; c++) {
      a[k] = v; // @step write
      k++;
    }
  }
}
