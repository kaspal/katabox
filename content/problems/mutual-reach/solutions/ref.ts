export function mutualReach(n: number, links: number[][]): number[] {
  const adj: number[][] = Array.from({ length: n }, () => []);
  const radj: number[][] = Array.from({ length: n }, () => []);
  for (const [a, b] of links) {
    adj[a].push(b);
    radj[b].push(a);
  }
  const seen = new Uint8Array(n);
  const order: number[] = [];
  const stackNode: number[] = [];
  const stackIdx: number[] = [];
  for (let s = 0; s < n; s++) {
    if (seen[s]) continue;
    seen[s] = 1;
    stackNode.push(s);
    stackIdx.push(0);
    while (stackNode.length > 0) {
      const top = stackNode.length - 1;
      const x = stackNode[top];
      if (stackIdx[top] < adj[x].length) {
        const y = adj[x][stackIdx[top]++];
        if (!seen[y]) {
          seen[y] = 1;
          stackNode.push(y);
          stackIdx.push(0);
        }
      } else {
        stackNode.pop();
        stackIdx.pop();
        order.push(x);
      }
    }
  }
  const group = new Int32Array(n).fill(-1);
  const sizes: number[] = [];
  for (let k = order.length - 1; k >= 0; k--) {
    const s = order[k];
    if (group[s] !== -1) continue;
    group[s] = sizes.length;
    let count = 0;
    const stack = [s];
    while (stack.length > 0) {
      const x = stack.pop()!;
      count++;
      for (const y of radj[x]) {
        if (group[y] === -1) {
          group[y] = sizes.length;
          stack.push(y);
        }
      }
    }
    sizes.push(count);
  }
  return sizes.sort((a, b) => a - b);
}
