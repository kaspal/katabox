const M = 8;
const EMPTY = -1;
const DELETED = -2;

class Table {
  slots: number[] = new Array<number>(M).fill(EMPTY);

  findSlot(key: number): number {
    let i = key % M; // @step hash
    while (this.slots[i] !== EMPTY && this.slots[i] !== key) {
      i = (i + 1) % M; // @step probe
    }
    return i;
  }

  insert(key: number): void {
    const i = this.findSlot(key);
    this.slots[i] = key; // @step place
  }

  find(key: number): number {
    return this.slots[this.findSlot(key)] === key ? 1 : 0; // @step found
  }

  delete(key: number): void {
    const i = this.findSlot(key);
    if (this.slots[i] === key) {
      this.slots[i] = DELETED; // @step tombstone
    }
  }
}

function run(ops: (string | number)[][]): number[] {
  const table = new Table();
  const out: number[] = [];
  for (const op of ops) {
    if (op[0] === "insert") table.insert(op[1] as number);
    else if (op[0] === "find") out.push(table.find(op[1] as number));
    else table.delete(op[1] as number);
  }
  return out;
}
