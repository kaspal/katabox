const grid: number[][] = Array.from({ length: 2 }, () => Array(3).fill(0));
grid[1][2] = 7;
const row = grid[0];
row[0] = 1;
for (const r of grid) {
  console.log(r.join(" "));
}
