export function minRooms(meetings: number[][]): number {
  const starts = meetings.map((m) => m[0]).sort((a, b) => a - b);
  const ends = meetings.map((m) => m[1]).sort((a, b) => a - b);
  let rooms = 0;
  let best = 0;
  let j = 0;
  for (const s of starts) {
    while (ends[j] <= s) {
      rooms--;
      j++;
    }
    rooms++;
    if (rooms > best) best = rooms;
  }
  return best;
}
