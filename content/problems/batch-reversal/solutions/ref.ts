// Provided by KataBox: the judge builds the input lists with this class.
export class ListNode {
  val: number;
  next: ListNode | null;
  constructor(val = 0, next: ListNode | null = null) {
    this.val = val;
    this.next = next;
  }
}

export function reverseBatches(line: ListNode | null, k: number): ListNode | null {
  const dummy = new ListNode(0, line);
  let before = dummy;
  for (;;) {
    let probe: ListNode | null = before;
    for (let i = 0; i < k; i++) {
      probe = probe!.next;
      if (probe === null) return dummy.next;
    }
    const first = before.next!;
    let prev: ListNode | null = null;
    let node: ListNode | null = first;
    for (let i = 0; i < k; i++) {
      const next: ListNode | null = node!.next;
      node!.next = prev;
      prev = node;
      node = next;
    }
    before.next = prev;
    first.next = node;
    before = first;
  }
}
