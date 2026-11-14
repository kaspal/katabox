function bucketSort(a: number[], count = 5): void {
  const buckets: number[][] = Array.from({ length: count }, () => []);
  for (const v of a) {
    buckets[Math.floor((v * count) / 100)].push(v); // @step scatter
  }
  let k = 0;
  for (const bucket of buckets) {
    bucket.sort((x, y) => x - y); // @step sort
    for (const v of bucket) {
      a[k] = v; // @step gather
      k++;
    }
  }
}
