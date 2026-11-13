// Provided by KataBox: the judge builds the input lists with this class.
export class ListNode {
  val: number;
  next: ListNode | null;
  constructor(val = 0, next: ListNode | null = null) {
    this.val = val;
    this.next = next;
  }
}

export function splitTrain(train: ListNode | null, k: number): (ListNode | null)[] {
  let n = 0;
  for (let node = train; node !== null; node = node.next) n++;
  const base = Math.floor(n / k);
  const extra = n % k;
  const parts: (ListNode | null)[] = [];
  let node = train;
  for (let i = 0; i < k; i++) {
    const size = base + (i < extra ? 1 : 0);
    if (size === 0) {
      parts.push(null);
      continue;
    }
    parts.push(node);
    for (let j = 0; j < size - 1; j++) node = node!.next;
    const next = node!.next;
    node!.next = null;
    node = next;
  }
  return parts;
}
