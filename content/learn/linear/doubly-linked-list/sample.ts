class DNode {
  value: number;
  prev: DNode | null = null;
  next: DNode | null = null;

  constructor(value: number) {
    this.value = value;
  }
}

class Deque {
  head: DNode | null = null;
  tail: DNode | null = null;

  pushFront(value: number): void {
    const node = new DNode(value);
    node.next = this.head; // @step link-front
    if (this.head !== null) this.head.prev = node;
    else this.tail = node;
    this.head = node;
  }

  pushBack(value: number): void {
    const node = new DNode(value);
    node.prev = this.tail; // @step link-back
    if (this.tail !== null) this.tail.next = node;
    else this.head = node;
    this.tail = node;
  }

  popFront(): number {
    if (this.head === null) {
      return -1; // @step empty-front
    }
    const node = this.head;
    this.head = node.next; // @step unlink-front
    if (this.head !== null) this.head.prev = null;
    else this.tail = null;
    return node.value;
  }

  popBack(): number {
    if (this.tail === null) {
      return -1; // @step empty-back
    }
    const node = this.tail;
    this.tail = node.prev; // @step unlink-back
    if (this.tail !== null) this.tail.next = null;
    else this.head = null;
    return node.value;
  }
}

function run(ops: (string | number)[][]): number[] {
  const d = new Deque();
  const out: number[] = [];
  for (const op of ops) {
    if (op[0] === "front") d.pushFront(op[1] as number);
    else if (op[0] === "back") d.pushBack(op[1] as number);
    else if (op[0] === "pop-front") out.push(d.popFront());
    else out.push(d.popBack());
  }
  out.push(0);
  for (let node = d.head; node !== null; node = node.next) out.push(node.value);
  out.push(0);
  for (let node = d.tail; node !== null; node = node.prev) out.push(node.value);
  return out;
}
