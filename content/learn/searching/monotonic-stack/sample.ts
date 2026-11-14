function nextGreater(a: number[]): number[] {
  const result = new Array<number>(a.length).fill(-1);
  const stack: number[] = [];
  for (let i = 0; i < a.length; i++) {
    while (stack.length > 0 && a[stack[stack.length - 1]] < a[i]) { // @step compare
      result[stack.pop()!] = a[i]; // @step pop
    }
    stack.push(i); // @step push
  }
  return result;
}
