class ListNode {
  value: number;
  next: ListNode | null = null;

  constructor(value: number) {
    this.value = value;
  }
}

function build(values: number[], pos: number): ListNode[] {
  const nodes = values.map((value) => new ListNode(value));
  for (let k = 0; k < nodes.length - 1; k++) nodes[k].next = nodes[k + 1];
  if (pos >= 0) nodes[nodes.length - 1].next = nodes[pos];
  return nodes;
}

function cycleStart(head: ListNode): ListNode | null {
  let slow: ListNode | null = head;
  let fast: ListNode | null = head;
  while (fast !== null && fast.next !== null) {
    slow = slow!.next;
    fast = fast.next.next; // @step move
    if (slow === fast) {
      break; // @step meet
    }
  }
  if (fast === null || fast.next === null) {
    return null; // @step no-cycle
  }
  slow = head; // @step reset
  while (slow !== fast) {
    slow = slow!.next;
    fast = fast!.next; // @step walk
  }
  return slow; // @step found
}

function run(values: number[], pos: number): number[] {
  const nodes = build(values, pos);
  const start = cycleStart(nodes[0]);
  return [start === null ? -1 : nodes.indexOf(start)];
}
