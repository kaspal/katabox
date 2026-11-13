export function hasTicket(seats: number[][], ticket: number): boolean {
  let r = 0;
  let c = seats[0].length - 1;
  while (r < seats.length && c >= 0) {
    const v = seats[r][c];
    if (v === ticket) return true;
    if (v > ticket) c--;
    else r++;
  }
  return false;
}
