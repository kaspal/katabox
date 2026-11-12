export function finishTime(jobs: number[], lathes: number): number {
  // A binary min-heap of the times at which each lathe becomes free.
  const heap: number[] = new Array(Math.min(lathes, jobs.length)).fill(0);
  let latest = 0;
  for (const job of jobs) {
    const end = heap[0] + job;
    // Replace the root and sift it down.
    let i = 0;
    const n = heap.length;
    for (;;) {
      const l = 2 * i + 1;
      const r = l + 1;
      let m = i;
      let value = end;
      if (l < n && heap[l] < value) {
        m = l;
        value = heap[l];
      }
      if (r < n && heap[r] < value) m = r;
      if (m === i) break;
      heap[i] = heap[m];
      i = m;
    }
    heap[i] = end;
    if (end > latest) latest = end;
  }
  return latest;
}
