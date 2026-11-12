export function generations(parent: number[]): number {
  const children: number[][] = parent.map(() => []);
  let founder = 0;
  parent.forEach((p, i) => {
    if (p < 0) founder = i;
    else children[p].push(i);
  });
  let level = [founder];
  let count = 0;
  while (level.length > 0) {
    count++;
    const next: number[] = [];
    for (const p of level) for (const c of children[p]) next.push(c);
    level = next;
  }
  return count;
}
