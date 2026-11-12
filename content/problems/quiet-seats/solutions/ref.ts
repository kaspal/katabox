export function quietSeats(rows: string[]): number {
  const h = rows.length;
  const w = rows[0].length;
  let count = 0;
  for (let r = 0; r < h; r++) {
    for (let c = 0; c < w; c++) {
      if (rows[r][c] !== ".") continue;
      let quiet = true;
      for (let i = Math.max(r - 1, 0); i <= Math.min(r + 1, h - 1) && quiet; i++) {
        for (let j = Math.max(c - 1, 0); j <= Math.min(c + 1, w - 1); j++) {
          if (rows[i][j] === "#") {
            quiet = false;
            break;
          }
        }
      }
      if (quiet) count++;
    }
  }
  return count;
}
