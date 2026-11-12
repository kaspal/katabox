export function firstMatches(words: string[], prefixes: string[]): string[] {
  const sorted = [...new Set(words)].sort();
  let total = 1;
  for (const w of sorted) total += w.length;
  // Node k's children are next[26k .. 26k + 25]; 0 means "no child" (the root is 0).
  const next = new Int32Array(26 * total);
  const best: string[] = [""];
  for (const w of sorted) {
    let node = 0;
    for (let i = 0; i < w.length; i++) {
      const at = 26 * node + (w.charCodeAt(i) - 97);
      if (next[at] === 0) {
        next[at] = best.length;
        best.push(w);
      }
      node = next[at];
    }
  }
  return prefixes.map((p) => {
    let node = 0;
    for (let i = 0; i < p.length; i++) {
      node = next[26 * node + (p.charCodeAt(i) - 97)];
      if (node === 0) return "";
    }
    return best[node];
  });
}
