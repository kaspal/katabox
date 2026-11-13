export function fuelStops(target: number, fuel: number, stations: number[][]): number {
  // A max-heap of the litres of passed stations.
  const heap: number[] = [];
  const push = (x: number) => {
    heap.push(x);
    let i = heap.length - 1;
    while (i > 0 && heap[(i - 1) >> 1] < heap[i]) {
      [heap[i], heap[(i - 1) >> 1]] = [heap[(i - 1) >> 1], heap[i]];
      i = (i - 1) >> 1;
    }
  };
  const pop = () => {
    const top = heap[0];
    const last = heap.pop()!;
    if (heap.length > 0) {
      heap[0] = last;
      let i = 0;
      for (;;) {
        const l = 2 * i + 1;
        const r = l + 1;
        let m = i;
        if (l < heap.length && heap[l] > heap[m]) m = l;
        if (r < heap.length && heap[r] > heap[m]) m = r;
        if (m === i) break;
        [heap[i], heap[m]] = [heap[m], heap[i]];
        i = m;
      }
    }
    return top;
  };
  let reach = fuel;
  let stops = 0;
  for (const [position, litres] of [...stations, [target, 0]]) {
    while (reach < position) {
      if (heap.length === 0) return -1;
      reach += pop();
      stops++;
    }
    push(litres);
  }
  return stops;
}
