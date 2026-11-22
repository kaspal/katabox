export function seatMap(rows: number, cols: number, bookings: number[][]): string[] {
  const hall = Array.from({ length: rows }, () => new Array<string>(cols).fill("."));
  for (const [r, c] of bookings) hall[r][c] = "X";
  return hall.map((row) => row.join(""));
}
