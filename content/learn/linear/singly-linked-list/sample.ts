class ListNode {
  value: number;
  next: ListNode | null = null;

  constructor(value: number) {
    this.value = value;
  }
}

function run(ops: (string | number)[][]): number[] {
  let head: ListNode | null = null;
  for (const op of ops) {
    if (op[0] === "front") {
      const node = new ListNode(op[1] as number);
      node.next = head; // @step link
      head = node; // @step new-head
    } else if (op[0] === "back") {
      const node = new ListNode(op[1] as number);
      if (head === null) {
        head = node; // @step first
      } else {
        let cur = head;
        while (cur.next !== null) {
          cur = cur.next; // @step walk
        }
        cur.next = node; // @step append
      }
    } else {
      let prev: ListNode | null = null;
      let cur: ListNode | null = head;
      while (cur !== null && cur.value !== op[1]) {
        [prev, cur] = [cur, cur.next]; // @step search
      }
      if (cur !== null) {
        if (prev === null) {
          head = cur.next; // @step unlink-head
        } else {
          prev.next = cur.next; // @step unlink
        }
      }
    }
  }
  const out: number[] = [];
  for (let node = head; node !== null; node = node.next) out.push(node.value);
  return out;
}
