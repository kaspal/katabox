export function topScores(scores: number[], k: number): number[] {
  // A min-heap of the best k scores so far.
  const heap: number[] = [];
  const down = (i: number) => {
    for (;;) {
      const l = 2 * i + 1;
      const r = l + 1;
      let m = i;
      if (l < heap.length && heap[l] < heap[m]) m = l;
      if (r < heap.length && heap[r] < heap[m]) m = r;
      if (m === i) return;
      [heap[i], heap[m]] = [heap[m], heap[i]];
      i = m;
    }
  };
  for (const s of scores) {
    if (heap.length < k) {
      heap.push(s);
      let i = heap.length - 1;
      while (i > 0) {
        const p = (i - 1) >> 1;
        if (heap[p] <= heap[i]) break;
        [heap[p], heap[i]] = [heap[i], heap[p]];
        i = p;
      }
    } else if (s > heap[0]) {
      heap[0] = s;
      down(0);
    }
  }
  return heap.sort((a, b) => b - a);
}
