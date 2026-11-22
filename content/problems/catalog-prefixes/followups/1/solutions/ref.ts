export function liveCounts(ops: string[], words: string[]): number[] {
  const children: Map<string, number>[] = [new Map()];
  const count = [0];
  const out: number[] = [];
  for (let i = 0; i < ops.length; i++) {
    const w = words[i];
    if (ops[i] === "count") {
      let node = 0;
      for (const c of w) {
        const nxt = children[node].get(c);
        if (nxt === undefined) {
          node = -1;
          break;
        }
        node = nxt;
      }
      out.push(node === -1 ? 0 : count[node]);
      continue;
    }
    const step = ops[i] === "add" ? 1 : -1;
    let node = 0;
    for (const c of w) {
      let nxt = children[node].get(c);
      if (nxt === undefined) {
        children.push(new Map());
        count.push(0);
        nxt = count.length - 1;
        children[node].set(c, nxt);
      }
      node = nxt;
      count[node] += step;
    }
  }
  return out;
}
