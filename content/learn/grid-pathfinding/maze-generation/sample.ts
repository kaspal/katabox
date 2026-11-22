const DIRS = [[-1, 0], [0, 1], [1, 0], [0, -1]];

function makeMaze(k: number, seed: number): number[] {
  const size = 2 * k + 1;
  const grid = Array.from({ length: size }, () =>
    new Array<number>(size).fill(1),
  );
  let state = seed;
  const rand = () => {
    state = (state * 48271) % 2147483647;
    return state;
  };
  grid[1][1] = 0;
  const stack = [[1, 1]];
  while (stack.length > 0) {
    const [r, c] = stack[stack.length - 1];
    const options: number[][] = [];
    for (const [dr, dc] of DIRS) {
      const nr = r + 2 * dr;
      const nc = c + 2 * dc;
      const inside = nr > 0 && nr < size && nc > 0 && nc < size;
      if (inside && grid[nr][nc] === 1) options.push([dr, dc]);
    }
    if (options.length > 0) {
      const [dr, dc] = options[rand() % options.length];
      grid[r + dr][c + dc] = 0;
      grid[r + 2 * dr][c + 2 * dc] = 0; // @step carve
      stack.push([r + 2 * dr, c + 2 * dc]);
    } else {
      stack.pop(); // @step back
    }
  }
  return grid.flat();
}
