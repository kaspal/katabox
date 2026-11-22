export function daysUntilWarmerAround(temps: number[]): number[] {
  const n = temps.length;
  const wait = new Array<number>(n).fill(0);
  const stack: number[] = [];
  for (let k = 0; k < 2 * n; k++) {
    const i = k % n;
    while (stack.length > 0 && temps[stack[stack.length - 1]] < temps[i]) {
      const d = stack.pop()!;
      wait[d] = (i - d + n) % n || n;
    }
    if (k < n) stack.push(i);
  }
  return wait;
}
