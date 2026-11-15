function runningTotals(a: number[]): number[] {
  const running = new Map<number, number>();
  const out: number[] = [];
  for (let i = 0; i + 1 < a.length; i += 2) {
    const [p, v] = [a[i], a[i + 1]];
    if (!running.has(p)) {
      running.set(p, v); // @step start
    } else {
      running.set(p, running.get(p)! + v); // @step add
    }
    out.push(running.get(p)!);
  }
  return out;
}
