function ahoCorasick(text: string, patterns: string[]): number[] {
  const go: Map<string, number>[] = [new Map()];
  const fail = [0];
  const out: number[][] = [[]];
  patterns.forEach((word, p) => {
    let node = 0;
    for (const ch of word) {
      if (!go[node].has(ch)) {
        go.push(new Map());
        fail.push(0);
        out.push([]);
        go[node].set(ch, go.length - 1);
      }
      node = go[node].get(ch)!;
    }
    out[node].push(p);
  });
  const queue = [...go[0].values()];
  while (queue.length > 0) {
    const node = queue.shift()!;
    for (const [ch, child] of go[node]) {
      let f = fail[node];
      while (f && !go[f].has(ch)) f = fail[f];
      fail[child] = go[f].get(ch) ?? 0; // @step link
      out[child] = [...out[child], ...out[fail[child]]];
      queue.push(child);
    }
  }
  const counts = new Array<number>(patterns.length).fill(0);
  let node = 0;
  for (const ch of text) {
    while (node && !go[node].has(ch)) {
      node = fail[node]; // @step follow
    }
    node = go[node].get(ch) ?? 0;
    for (const p of out[node]) {
      counts[p]++; // @step hit
    }
  }
  return counts;
}
