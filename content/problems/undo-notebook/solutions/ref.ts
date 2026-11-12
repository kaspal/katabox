type Change = [number, number | null, number];

export function notebook(ops: string[], args: number[][]): number[] {
  const values = new Map<number, number>();
  const undo: Change[] = [];
  const redo: Change[] = [];
  const set = (key: number, value: number | null) => {
    if (value === null) values.delete(key);
    else values.set(key, value);
  };
  const reports: number[] = [];
  ops.forEach((op, i) => {
    const a = args[i];
    if (op === "write") {
      undo.push([a[0], values.get(a[0]) ?? null, a[1]]);
      values.set(a[0], a[1]);
      redo.length = 0;
    } else if (op === "read") {
      reports.push(values.get(a[0]) ?? -1);
    } else if (op === "undo") {
      const c = undo.pop();
      if (c) {
        set(c[0], c[1]);
        redo.push(c);
      }
    } else {
      const c = redo.pop();
      if (c) {
        set(c[0], c[2]);
        undo.push(c);
      }
    }
  });
  return reports;
}
