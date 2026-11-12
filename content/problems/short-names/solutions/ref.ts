export function shortNames(names: string[]): string[] {
  let total = 1;
  for (const s of names) total += s.length;
  // Node k's children are next[26k .. 26k + 25]; 0 means "no child" (the root is 0).
  const next = new Int32Array(26 * total);
  const passes = new Int32Array(total);
  let size = 1;
  for (const s of names) {
    let node = 0;
    for (let i = 0; i < s.length; i++) {
      const at = 26 * node + (s.charCodeAt(i) - 97);
      if (next[at] === 0) next[at] = size++;
      node = next[at];
      passes[node]++;
    }
  }
  return names.map((s) => {
    let node = 0;
    for (let i = 0; i < s.length; i++) {
      node = next[26 * node + (s.charCodeAt(i) - 97)];
      if (passes[node] === 1) return s.slice(0, i + 1);
    }
    return s;
  });
}
