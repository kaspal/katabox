export function shortestRoute(grid: number[][]): number {
  const rows = grid.length;
  const cols = grid[0].length;
  if (grid[0][0] === 1 || grid[rows - 1][cols - 1] === 1) return -1;
  const dist = new Int32Array(rows * cols).fill(-1);
  const queue = new Int32Array(rows * cols);
  let head = 0;
  let tail = 0;
  dist[0] = 0;
  queue[tail++] = 0;
  const moves = [[1, 0], [-1, 0], [0, 1], [0, -1]];
  while (head < tail) {
    const cell = queue[head++];
    const r = Math.floor(cell / cols);
    const c = cell % cols;
    if (r === rows - 1 && c === cols - 1) return dist[cell];
    for (const [dr, dc] of moves) {
      const nr = r + dr;
      const nc = c + dc;
      if (nr < 0 || nr >= rows || nc < 0 || nc >= cols || grid[nr][nc] !== 0) continue;
      const next = nr * cols + nc;
      if (dist[next] !== -1) continue;
      dist[next] = dist[cell] + 1;
      queue[tail++] = next;
    }
  }
  return -1;
}
