const WHITE = 0;
const GREY = 1;
const BLACK = 2;

function hasCycle(n: number, adj: number[][][]): number {
  const color = new Array<number>(n).fill(WHITE);

  function visit(u: number): boolean {
    color[u] = GREY; // @step grey
    for (const [v] of adj[u]) {
      if (color[v] === GREY) {
        return true; // @step back-edge
      }
      if (color[v] === WHITE && visit(v)) return true;
    }
    color[u] = BLACK; // @step black
    return false;
  }

  for (let u = 0; u < n; u++) {
    if (color[u] === WHITE && visit(u)) return 1;
  }
  return 0;
}
