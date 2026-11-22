const grid: number[][] = Array(2).fill(Array(3).fill(0));
grid[0][1] = 5;
for (const row of grid) {
  console.log(row.join(" "));
}
