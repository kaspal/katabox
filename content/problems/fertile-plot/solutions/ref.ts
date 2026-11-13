import { readFileSync } from "node:fs";

const tokens = readFileSync(0, "utf8").split(/\s+/).filter((t) => t.length > 0);
let rows = Number(tokens[0]);
let cols = Number(tokens[1]);
let grid = new Int32Array(rows * cols);
for (let i = 0; i < rows * cols; i++) grid[i] = Number(tokens[2 + i]);
if (rows > cols) {
  // Put the shorter side in the outer loops.
  const flipped = new Int32Array(rows * cols);
  for (let i = 0; i < rows; i++) {
    for (let j = 0; j < cols; j++) flipped[j * rows + i] = grid[i * cols + j];
  }
  grid = flipped;
  [rows, cols] = [cols, rows];
}
let best = -Infinity;
const col = new Int32Array(cols);
for (let top = 0; top < rows; top++) {
  col.fill(0);
  for (let bottom = top; bottom < rows; bottom++) {
    const base = bottom * cols;
    let cur = 0; // Kadane: the best run of columns ending at j
    for (let j = 0; j < cols; j++) {
      const x = col[j] + grid[base + j];
      col[j] = x;
      cur = cur > 0 ? cur + x : x;
      if (cur > best) best = cur;
    }
  }
}
process.stdout.write(`${best}\n`);
