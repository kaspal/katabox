const DIRS = [[-1, 0], [0, 1], [1, 0], [0, -1]];

function cost(cell: number): number {
  return cell >= 2 ? cell : 1;
}

// The queue keeps (h, row, column) entries sorted, smallest first.
function greedyBestFirst(grid: number[][], start: number[], goal: number[]): number[] {
  const rows = grid.length;
  const cols = grid[0].length;
  const h = (r: number, c: number) => Math.abs(r - goal[0]) + Math.abs(c - goal[1]);
  const paid: (number | null)[][] = Array.from({ length: rows }, () => new Array(cols).fill(null));
  paid[start[0]][start[1]] = 0;
  const queue = [[h(start[0], start[1]), start[0], start[1]]];
  while (queue.length > 0) {
    const [, r, c] = queue.shift()!; // @step pop
    if (r === goal[0] && c === goal[1]) {
      return [paid[r][c]!]; // @step goal
    }
    for (const [dr, dc] of DIRS) {
      const nr = r + dr;
      const nc = c + dc;
      if (nr >= 0 && nr < rows && nc >= 0 && nc < cols && grid[nr][nc] !== 1 && paid[nr][nc] === null) {
        paid[nr][nc] = paid[r][c]! + cost(grid[nr][nc]); // @step discover
        queue.push([h(nr, nc), nr, nc]);
        queue.sort((x, y) => x[0] - y[0] || x[1] - y[1] || x[2] - y[2]);
      }
    }
  }
  return [-1];
}
