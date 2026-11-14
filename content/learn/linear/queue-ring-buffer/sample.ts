class RingBuffer {
  data: number[];
  head = 0;
  size = 0;

  constructor(capacity: number) {
    this.data = new Array<number>(capacity).fill(0);
  }

  enqueue(value: number): boolean {
    if (this.size === this.data.length) {
      return false; // @step full
    }
    const tail = (this.head + this.size) % this.data.length;
    this.data[tail] = value; // @step enqueue
    this.size++;
    return true;
  }

  dequeue(): number {
    if (this.size === 0) {
      return -1; // @step empty
    }
    const value = this.data[this.head]; // @step dequeue
    this.head = (this.head + 1) % this.data.length;
    this.size--;
    return value;
  }
}

function run(capacity: number, ops: (string | number)[][]): number[] {
  const queue = new RingBuffer(capacity);
  const out: number[] = [];
  for (const op of ops) {
    if (op[0] === "enqueue") {
      queue.enqueue(op[1] as number);
    } else {
      out.push(queue.dequeue());
    }
  }
  return out;
}
