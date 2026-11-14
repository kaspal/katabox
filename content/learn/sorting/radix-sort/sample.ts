function radixSort(a: number[]): void {
  for (let place = 1; place <= Math.max(...a); place *= 10) { // @step pass
    const buckets: number[][] = Array.from({ length: 10 }, () => []);
    for (const v of a) {
      buckets[Math.floor(v / place) % 10].push(v); // @step distribute
    }
    let k = 0;
    for (const bucket of buckets) {
      for (const v of bucket) {
        a[k] = v; // @step collect
        k++;
      }
    }
  }
}
