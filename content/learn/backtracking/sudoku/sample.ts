function solveSudoku(size: number, cells: number[]): number[] {
  const box = Math.round(Math.sqrt(size));
  const grid = [...cells];
  const ok = (at: number, d: number): boolean => {
    const r = Math.floor(at / size);
    const c = at % size;
    for (let k = 0; k < size; k++) {
      if (grid[r * size + k] === d || grid[k * size + c] === d) return false;
    }
    const top = r - (r % box);
    const left = c - (c % box);
    for (let rr = top; rr < top + box; rr++) {
      for (let cc = left; cc < left + box; cc++) {
        if (grid[rr * size + cc] === d) return false;
      }
    }
    return true;
  };
  const fill = (): boolean => {
    const at = grid.indexOf(0);
    if (at === -1) return true;
    for (let d = 1; d <= size; d++) {
      if (ok(at, d)) {
        grid[at] = d; // @step place
        if (fill()) return true;
        grid[at] = 0; // @step undo
      }
    }
    return false;
  };
  return fill() ? grid : [-1];
}
