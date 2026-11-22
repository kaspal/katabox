class DynArray {
  data: number[] = [0, 0];
  size = 0;

  insert(i: number, value: number): void {
    if (this.size === this.data.length) {
      const room = new Array<number>(this.data.length).fill(0);
      this.data = [...this.data, ...room]; // @step grow
    }
    for (let k = this.size; k > i; k--) {
      this.data[k] = this.data[k - 1]; // @step shift-right
    }
    this.data[i] = value; // @step write
    this.size++;
  }

  remove(i: number): void {
    for (let k = i; k < this.size - 1; k++) {
      this.data[k] = this.data[k + 1]; // @step shift-left
    }
    this.size--; // @step shrink
  }
}

function run(ops: (string | number)[][]): number[] {
  const arr = new DynArray();
  for (const op of ops) {
    if (op[0] === "append") {
      arr.insert(arr.size, op[1] as number);
    } else if (op[0] === "insert") {
      arr.insert(Math.min(op[1] as number, arr.size), op[2] as number);
    } else if (arr.size > 0) {
      arr.remove(Math.min(op[1] as number, arr.size - 1));
    }
  }
  return arr.data.slice(0, arr.size);
}
