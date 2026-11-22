function build(
  n: number,
  directed: number,
  edges: number[][],
): [number[][], number[][]] {
  const adj: number[][] = Array.from({ length: n }, () => []);
  const matrix: number[][] = Array.from({ length: n }, () =>
    new Array<number>(n).fill(0),
  );
  for (const [u, v, w] of edges) {
    adj[u].push(v); // @step list
    matrix[u][v] = w; // @step matrix
    if (!directed) {
      adj[v].push(u); // @step list-back
      matrix[v][u] = w;
    }
  }
  return [adj, matrix];
}

function run(n: number, directed: number, edges: number[][]): number[] {
  const [adj, matrix] = build(n, directed, edges);
  const out: number[] = [];
  for (const row of adj) out.push(row.length, ...row);
  for (const row of matrix) out.push(...row);
  return out;
}
