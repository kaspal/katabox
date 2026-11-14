function push(heap: number[], value: number): void {
  heap.push(value); // @step append
  let i = heap.length - 1;
  while (i > 0) {
    const parent = Math.floor((i - 1) / 2);
    if (heap[parent] <= heap[i]) { // @step compare-up
      break;
    }
    [heap[parent], heap[i]] = [heap[i], heap[parent]]; // @step swap-up
    i = parent;
  }
}

function pop(heap: number[]): number {
  if (heap.length === 0) {
    return -1; // @step empty
  }
  const top = heap[0]; // @step take
  const last = heap.pop()!;
  if (heap.length > 0) {
    heap[0] = last; // @step move-last
    let i = 0;
    while (2 * i + 1 < heap.length) {
      let child = 2 * i + 1;
      if (child + 1 < heap.length && heap[child + 1] < heap[child]) child++;
      if (heap[i] <= heap[child]) { // @step compare-down
        break;
      }
      [heap[i], heap[child]] = [heap[child], heap[i]]; // @step swap-down
      i = child;
    }
  }
  return top;
}

function run(ops: (string | number)[][]): number[] {
  const heap: number[] = [];
  const out: number[] = [];
  for (const op of ops) {
    if (op[0] === "push") push(heap, op[1] as number);
    else out.push(pop(heap));
  }
  return out;
}
