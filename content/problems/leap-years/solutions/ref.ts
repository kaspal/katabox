export function leapYears(start: number, end: number): number {
  let count = 0;
  for (let y = start; y <= end; y++) {
    if (y % 4 === 0 && (y % 100 !== 0 || y % 400 === 0)) count++;
  }
  return count;
}
