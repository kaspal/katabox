import { readFileSync } from "node:fs";

const lines = readFileSync(0, "utf8").split(/\s+/).filter((t) => t.length > 0);
const r = Number(lines[0]);
const c = Number(lines[1]);
// height[x]: how many free squares stand in column x, ending at the current row. Each
// row then is a skyline, and the best banner whose bottom edge is on this row is the
// largest rectangle under it: a stack of columns of increasing height.
const height = new Int32Array(c + 1); // a 0 at the end empties the stack
const stack = new Int32Array(c + 1);
let best = 0;
for (let y = 0; y < r; y++) {
  const row = lines[2 + y];
  for (let x = 0; x < c; x++) height[x] = row.charCodeAt(x) === 46 ? height[x] + 1 : 0; // 46 is "."
  let top = 0; // stack size
  for (let x = 0; x <= c; x++) {
    const h = height[x];
    while (top > 0 && height[stack[top - 1]] >= h) {
      const tall = height[stack[--top]];
      const left = top > 0 ? stack[top - 1] + 1 : 0;
      if (tall * (x - left) > best) best = tall * (x - left);
    }
    stack[top++] = x;
  }
}
process.stdout.write(`${best}\n`);
