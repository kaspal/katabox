export function roadRepairs(length: number, repairs: number[][]): number[] {
  const change = new Array<number>(length + 1).fill(0);
  for (const [a, b, crews] of repairs) {
    change[a] += crews;
    change[b + 1] -= crews;
  }
  const out: number[] = [];
  let running = 0;
  for (let i = 0; i < length; i++) {
    running += change[i];
    out.push(running);
  }
  return out;
}
