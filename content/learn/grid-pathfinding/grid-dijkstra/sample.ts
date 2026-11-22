const DIRS = [[-1, 0], [0, 1], [1, 0], [0, -1]];

function cost(cell: number): number {
  return cell >= 2 ? cell : 1;
}

// TypeScript has no built-in priority queue: this one keeps (distance, row,
// column) entries sorted, smallest first, which is enough for a small grid.
function gridDijkstra(
  grid: number[][],
  start: number[],
  goal: number[],
): number[] {
  const rows = grid.length;
  const cols = grid[0].length;
  const dist: (number | null)[][] = Array.from({ length: rows }, () =>
    new Array(cols).fill(null),
  );
  dist[start[0]][start[1]] = 0;
  const queue = [[0, start[0], start[1]]];
  while (queue.length > 0) {
    const [d, r, c] = queue.shift()!; // @step pop
    if (d > dist[r][c]!) {
      continue; // @step stale
    }
    if (r === goal[0] && c === goal[1]) {
      return [d]; // @step goal
    }
    for (const [dr, dc] of DIRS) {
      const nr = r + dr;
      const nc = c + dc;
      if (nr >= 0 && nr < rows && nc >= 0 && nc < cols && grid[nr][nc] !== 1) {
        const nd = d + cost(grid[nr][nc]);
        if (dist[nr][nc] === null || nd < dist[nr][nc]!) {
          dist[nr][nc] = nd; // @step relax
          queue.push([nd, nr, nc]);
          queue.sort((x, y) => x[0] - y[0] || x[1] - y[1] || x[2] - y[2]);
        }
      }
    }
  }
  return [-1];
}
