export function daysUntilWarmer(temps: number[]): number[] {
  const wait: number[] = new Array(temps.length).fill(0);
  const stack: number[] = [];
  for (let i = 0; i < temps.length; i++) {
    while (stack.length > 0 && temps[stack[stack.length - 1]] < temps[i]) {
      const j = stack.pop()!;
      wait[j] = i - j;
    }
    stack.push(i);
  }
  return wait;
}
