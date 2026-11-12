export function prefixCounts(titles: string[], typed: string[]): number[] {
  // Node k's children are next[26k .. 26k + 25]; 0 means "no child" (the root is 0).
  let total = 1;
  for (const t of titles) total += t.length;
  const next = new Int32Array(26 * total);
  const passes = new Int32Array(total);
  let size = 1;
  for (const t of titles) {
    let node = 0;
    for (let i = 0; i < t.length; i++) {
      const at = 26 * node + (t.charCodeAt(i) - 97);
      if (next[at] === 0) next[at] = size++;
      node = next[at];
      passes[node]++;
    }
  }
  return typed.map((p) => {
    let node = 0;
    for (let i = 0; i < p.length; i++) {
      node = next[26 * node + (p.charCodeAt(i) - 97)];
      if (node === 0) return 0;
    }
    return passes[node];
  });
}
