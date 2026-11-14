function run(ops: (string | number)[][]): number[] {
  const stack: number[] = [];
  const out: number[] = [];
  for (const op of ops) {
    if (op[0] === "push") {
      stack.push(op[1] as number); // @step push
    } else if (op[0] === "pop") {
      out.push(stack.length > 0 ? stack.pop()! : -1); // @step pop
    } else {
      out.push(stack.length > 0 ? stack[stack.length - 1] : -1); // @step peek
    }
  }
  return out;
}
