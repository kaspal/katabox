export function pondFences(ice: string[]): number[] {
  const rows = ice.length;
  const cols = ice[0].length;
  const seen = new Uint8Array(rows * cols);
  const fences: number[] = [];
  for (let r = 0; r < rows; r++)
    for (let c = 0; c < cols; c++) {
      if (ice[r][c] !== "#" || seen[r * cols + c]) continue;
      seen[r * cols + c] = 1;
      const stack = [r * cols + c];
      let edges = 0;
      while (stack.length > 0) {
        const cell = stack.pop()!;
        const y = Math.floor(cell / cols);
        const x = cell % cols;
        for (const [ny, nx] of [
          [y + 1, x],
          [y - 1, x],
          [y, x + 1],
          [y, x - 1],
        ]) {
          if (ny >= 0 && ny < rows && nx >= 0 && nx < cols && ice[ny][nx] === "#") {
            if (!seen[ny * cols + nx]) {
              seen[ny * cols + nx] = 1;
              stack.push(ny * cols + nx);
            }
          } else edges++;
        }
      }
      fences.push(edges);
    }
  return fences.sort((a, b) => b - a);
}
