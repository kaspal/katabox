// Provided by KataBox: the judge builds the input lists with this class.
export class ListNode {
  val: number;
  next: ListNode | null;
  constructor(val = 0, next: ListNode | null = null) {
    this.val = val;
    this.next = next;
  }
}

export function mergeDistinct(lines: (ListNode | null)[]): ListNode | null {
  const tickets: number[] = [];
  for (let node of lines) {
    while (node !== null) {
      tickets.push(node.val);
      node = node.next;
    }
  }
  tickets.sort((a, b) => a - b);
  const dummy = new ListNode();
  let tail = dummy;
  for (let i = 0; i < tickets.length; i++) {
    if (i > 0 && tickets[i] === tickets[i - 1]) continue;
    tail.next = new ListNode(tickets[i]);
    tail = tail.next;
  }
  return dummy.next;
}
