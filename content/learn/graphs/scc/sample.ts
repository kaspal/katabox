function kosaraju(n: number, adj: number[][][]): number[] {
  const seen = new Array<boolean>(n).fill(false);
  const finished: number[] = [];
  const first = (u: number): void => {
    seen[u] = true;
    for (const [v] of adj[u]) {
      if (!seen[v]) first(v);
    }
    finished.push(u); // @step finish
  };
  for (let u = 0; u < n; u++) if (!seen[u]) first(u);

  const back: number[][] = Array.from({ length: n }, () => []);
  for (let u = 0; u < n; u++) for (const [v] of adj[u]) back[v].push(u);
  const label = new Array<number>(n).fill(-1);
  const second = (u: number, c: number): void => {
    label[u] = c; // @step assign
    for (const v of back[u]) {
      if (label[v] === -1) second(v, c);
    }
  };
  let count = 0;
  for (const u of [...finished].reverse()) {
    if (label[u] === -1) {
      second(u, count);
      count++;
    }
  }
  return label;
}
