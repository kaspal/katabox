export function fewestBlocks(plays: number[], requests: number[][]): number[] {
  const n = plays.length;
  const start = new Array<number>(n).fill(0);
  const last = new Map<number, number>();
  let left = 0;
  for (let r = 0; r < n; r++) {
    const prev = last.get(plays[r]);
    if (prev !== undefined && prev >= left) left = prev + 1;
    last.set(plays[r], r);
    start[r] = left;
  }
  const next = new Array<number>(n + 1).fill(n);
  let r = 0;
  for (let i = 0; i < n; i++) {
    while (r < n && start[r] <= i) r++;
    next[i] = r;
  }
  const jump: number[][] = [next];
  while (1 << jump.length <= n) {
    const prev = jump[jump.length - 1];
    jump.push(prev.map((p) => prev[p]));
  }
  return requests.map(([a, b]) => {
    let pos = a;
    let blocks = 1;
    for (let k = jump.length - 1; k >= 0; k--) {
      if (jump[k][pos] <= b) {
        pos = jump[k][pos];
        blocks += 2 ** k;
      }
    }
    return blocks;
  });
}
