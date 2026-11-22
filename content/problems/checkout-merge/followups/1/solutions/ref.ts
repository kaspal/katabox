// Provided by KataBox: the judge builds the input lists with this class.
export class ListNode {
  val: number;
  next: ListNode | null;
  constructor(val = 0, next: ListNode | null = null) {
    this.val = val;
    this.next = next;
  }
}

export function fairSample(lines: (ListNode | null)[]): number[] {
  // A binary min-heap of [ticket, line index, node].
  const heap: [number, number, ListNode][] = [];
  const less = (a: [number, number, ListNode], b: [number, number, ListNode]) =>
    a[0] < b[0] || (a[0] === b[0] && a[1] < b[1]);
  const push = (item: [number, number, ListNode]) => {
    heap.push(item);
    let i = heap.length - 1;
    while (i > 0) {
      const parent = (i - 1) >> 1;
      if (!less(heap[i], heap[parent])) break;
      [heap[i], heap[parent]] = [heap[parent], heap[i]];
      i = parent;
    }
  };
  const pop = (): [number, number, ListNode] => {
    const top = heap[0];
    const last = heap.pop()!;
    if (heap.length > 0) {
      heap[0] = last;
      let i = 0;
      for (;;) {
        const l = 2 * i + 1;
        const r = l + 1;
        let m = i;
        if (l < heap.length && less(heap[l], heap[m])) m = l;
        if (r < heap.length && less(heap[r], heap[m])) m = r;
        if (m === i) break;
        [heap[i], heap[m]] = [heap[m], heap[i]];
        i = m;
      }
    }
    return top;
  };
  let largest = 0;
  lines.forEach((node, i) => {
    push([node!.val, i, node!]);
    largest = Math.max(largest, node!.val);
  });
  let best: number[] | null = null;
  for (;;) {
    const [low, i, node] = pop();
    if (best === null || largest - low < best[1] - best[0]) best = [low, largest];
    if (node.next === null) return best;
    const next = node.next;
    largest = Math.max(largest, next.val);
    push([next.val, i, next]);
  }
}
