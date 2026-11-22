function f(n: number): number {
  const heap: number[] = [];
  for (let i = 0; i < n; i++) {
    heap.push((i * 37) % n);
  }
  for (let start = Math.floor(n / 2) - 1; start >= 0; start--) {
    siftDown(heap, start, n);
  }
  return heap[0];
}

function siftDown(heap: number[], i: number, size: number): void {
  while (2 * i + 1 < size) {
    let child = 2 * i + 1;
    if (child + 1 < size && heap[child + 1] > heap[child]) {
      child++;
    }
    if (heap[i] >= heap[child]) {
      return;
    }
    [heap[i], heap[child]] = [heap[child], heap[i]];
    i = child;
  }
}
