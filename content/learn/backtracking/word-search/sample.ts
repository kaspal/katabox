const DIRS = [[-1, 0], [0, 1], [1, 0], [0, -1]];

function findWord(
  rows: number,
  cols: number,
  letters: string,
  word: string,
): number[] {
  const path: number[][] = [];
  const walk = (r: number, c: number, k: number): boolean => {
    if (letters[r * cols + c] !== word[k]) return false;
    if (path.some(([pr, pc]) => pr === r && pc === c)) return false;
    path.push([r, c]); // @step step
    if (k === word.length - 1) return true;
    for (const [dr, dc] of DIRS) {
      const nr = r + dr;
      const nc = c + dc;
      const inside = nr >= 0 && nr < rows && nc >= 0 && nc < cols;
      if (inside && walk(nr, nc, k + 1)) return true;
    }
    path.pop(); // @step back
    return false;
  };
  for (let r = 0; r < rows; r++) {
    for (let c = 0; c < cols; c++) {
      if (walk(r, c, 0)) return [1, ...path.flat()];
    }
  }
  return [0];
}
