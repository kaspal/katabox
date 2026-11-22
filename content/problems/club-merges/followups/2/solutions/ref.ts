export function clubsWithUndo(n: number, ops: number[][]): number[] {
  const parent = Array.from({ length: n }, (_, i) => i);
  const size = new Array<number>(n).fill(1);
  const history: number[] = [];
  let clubs = n;
  const find = (x: number) => {
    while (parent[x] !== x) x = parent[x];
    return x;
  };
  return ops.map((op) => {
    if (op[0] === 0) {
      if (history.length > 0) {
        const rb = history.pop()!;
        if (rb !== -1) {
          const ra = parent[rb];
          size[ra] -= size[rb];
          parent[rb] = rb;
          clubs++;
        }
      }
    } else {
      let ra = find(op[1]);
      let rb = find(op[2]);
      if (ra === rb) history.push(-1);
      else {
        if (size[ra] < size[rb]) [ra, rb] = [rb, ra];
        parent[rb] = ra;
        size[ra] += size[rb];
        clubs--;
        history.push(rb);
      }
    }
    return clubs;
  });
}
