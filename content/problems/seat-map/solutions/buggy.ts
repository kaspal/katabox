export function seatMap(rows: number, cols: number, bookings: number[][]): string[] {
  const hall = new Array<string[]>(rows).fill(new Array<string>(cols).fill("."));
  for (const [r, c] of bookings) hall[r][c] = "X";
  return hall.map((row) => row.join(""));
}
