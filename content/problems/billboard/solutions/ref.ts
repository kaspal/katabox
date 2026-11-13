export function largestPoster(panels: number[]): number {
  const stack: number[] = [];
  let best = 0;
  for (let i = 0; i <= panels.length; i++) {
    const h = i < panels.length ? panels[i] : 0;
    while (stack.length > 0 && panels[stack[stack.length - 1]] > h) {
      const k = stack.pop()!;
      const width = stack.length > 0 ? i - stack[stack.length - 1] - 1 : i;
      best = Math.max(best, panels[k] * width);
    }
    stack.push(i);
  }
  return best;
}
