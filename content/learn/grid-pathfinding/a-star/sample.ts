const DIRS = [[-1, 0], [0, 1], [1, 0], [0, -1]];

function cost(cell: number): number {
  return cell >= 2 ? cell : 1;
}

// The queue keeps (f, h, row, column) entries sorted, smallest first.
function aStar(grid: number[][], start: number[], goal: number[]): number[] {
  const rows = grid.length;
  const cols = grid[0].length;
  const h = (r: number, c: number) =>
    Math.abs(r - goal[0]) + Math.abs(c - goal[1]);
  const g: (number | null)[][] = Array.from({ length: rows }, () =>
    new Array(cols).fill(null),
  );
  g[start[0]][start[1]] = 0;
  const first = h(start[0], start[1]);
  const queue = [[first, first, start[0], start[1]]];
  while (queue.length > 0) {
    const [f, hr, r, c] = queue.shift()!; // @step pop
    if (f - hr > g[r][c]!) {
      continue; // @step stale
    }
    if (r === goal[0] && c === goal[1]) {
      return [g[r][c]!]; // @step goal
    }
    for (const [dr, dc] of DIRS) {
      const nr = r + dr;
      const nc = c + dc;
      if (nr >= 0 && nr < rows && nc >= 0 && nc < cols && grid[nr][nc] !== 1) {
        const ng = g[r][c]! + cost(grid[nr][nc]);
        if (g[nr][nc] === null || ng < g[nr][nc]!) {
          g[nr][nc] = ng; // @step relax
          queue.push([ng + h(nr, nc), h(nr, nc), nr, nc]);
          queue.sort(
            (x, y) => x[0] - y[0] || x[1] - y[1] || x[2] - y[2] || x[3] - y[3],
          );
        }
      }
    }
  }
  return [-1];
}
