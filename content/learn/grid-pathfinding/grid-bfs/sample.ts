const DIRS = [[-1, 0], [0, 1], [1, 0], [0, -1]];

function gridBfs(grid: number[][], start: number[], goal: number[]): number[] {
  const rows = grid.length;
  const cols = grid[0].length;
  const dist = Array.from({ length: rows }, () =>
    new Array<number>(cols).fill(-1),
  );
  dist[start[0]][start[1]] = 0;
  const queue = [start];
  let head = 0;
  while (head < queue.length) {
    const [r, c] = queue[head++]; // @step visit
    if (r === goal[0] && c === goal[1]) {
      return [dist[r][c]]; // @step goal
    }
    for (const [dr, dc] of DIRS) {
      const nr = r + dr;
      const nc = c + dc;
      const inside = nr >= 0 && nr < rows && nc >= 0 && nc < cols;
      if (inside && grid[nr][nc] !== 1 && dist[nr][nc] === -1) {
        dist[nr][nc] = dist[r][c] + 1; // @step discover
        queue.push([nr, nc]);
      }
    }
  }
  return [-1];
}
