class ListNode {
  value: number;
  next: ListNode | null = null;

  constructor(value: number) {
    this.value = value;
  }
}

function build(values: number[]): ListNode | null {
  let head: ListNode | null = null;
  for (let k = values.length - 1; k >= 0; k--) {
    const node = new ListNode(values[k]);
    node.next = head;
    head = node;
  }
  return head;
}

function reverse(head: ListNode | null): ListNode | null {
  let prev: ListNode | null = null;
  let cur = head;
  while (cur !== null) {
    const nxt: ListNode | null = cur.next; // @step save
    cur.next = prev; // @step flip
    [prev, cur] = [cur, nxt]; // @step advance
  }
  return prev; // @step new-head
}

function run(values: number[]): number[] {
  const out: number[] = [];
  for (let node = reverse(build(values)); node !== null; node = node.next) {
    out.push(node.value);
  }
  return out;
}
