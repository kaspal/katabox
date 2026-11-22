function f(n: number): number {
  const stack: number[] = [0];
  let visited = 0;
  while (stack.length > 0) {
    const node = stack.pop() as number;
    visited++;
    const right = 2 * node + 2;
    if (right < n) {
      stack.push(right);
    }
    const left = 2 * node + 1;
    if (left < n) {
      stack.push(left);
    }
  }
  return visited;
}
