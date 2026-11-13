// Provided by KataBox: the judge builds the input lists with this class.
export class ListNode {
  val: number;
  next: ListNode | null;
  constructor(val = 0, next: ListNode | null = null) {
    this.val = val;
    this.next = next;
  }
}

export function zigzagBoarding(queue: ListNode | null): ListNode | null {
  if (queue === null || queue.next === null) return queue;
  let slow: ListNode = queue;
  let fast: ListNode | null = queue.next;
  while (fast !== null && fast.next !== null) {
    slow = slow.next!;
    fast = fast.next.next;
  }
  let back = slow.next;
  slow.next = null;
  let prev: ListNode | null = null;
  while (back !== null) {
    const next: ListNode | null = back.next;
    back.next = prev;
    prev = back;
    back = next;
  }
  let front: ListNode | null = queue;
  back = prev;
  while (back !== null && front !== null) {
    const frontNext: ListNode | null = front.next;
    const backNext: ListNode | null = back.next;
    front.next = back;
    back.next = frontNext;
    front = frontNext;
    back = backNext;
  }
  return queue;
}
