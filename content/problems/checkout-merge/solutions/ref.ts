// Provided by KataBox: the judge builds the input lists with this class.
export class ListNode {
  val: number;
  next: ListNode | null;
  constructor(val = 0, next: ListNode | null = null) {
    this.val = val;
    this.next = next;
  }
}

export function mergeLines(lines: (ListNode | null)[]): ListNode | null {
  const fronts = [...lines];
  // A binary min-heap of line indexes, ordered by (front ticket, index).
  const heap: number[] = [];
  const less = (a: number, b: number) => fronts[a]!.val < fronts[b]!.val || (fronts[a]!.val === fronts[b]!.val && a < b);
  const push = (i: number) => {
    heap.push(i);
    let c = heap.length - 1;
    while (c > 0) {
      const p = (c - 1) >> 1;
      if (!less(heap[c], heap[p])) break;
      [heap[c], heap[p]] = [heap[p], heap[c]];
      c = p;
    }
  };
  const pop = (): number => {
    const top = heap[0];
    const last = heap.pop()!;
    if (heap.length > 0) {
      heap[0] = last;
      let p = 0;
      for (;;) {
        const l = 2 * p + 1;
        const r = l + 1;
        let m = p;
        if (l < heap.length && less(heap[l], heap[m])) m = l;
        if (r < heap.length && less(heap[r], heap[m])) m = r;
        if (m === p) break;
        [heap[m], heap[p]] = [heap[p], heap[m]];
        p = m;
      }
    }
    return top;
  };
  fronts.forEach((node, i) => {
    if (node !== null) push(i);
  });
  const dummy = new ListNode();
  let tail = dummy;
  while (heap.length > 0) {
    const i = pop();
    const node = fronts[i]!;
    fronts[i] = node.next;
    tail.next = node;
    tail = node;
    if (fronts[i] !== null) push(i);
  }
  tail.next = null;
  return dummy.next;
}
