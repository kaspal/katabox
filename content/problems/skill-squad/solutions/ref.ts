export function smallestSquad(skills: number, people: number[]): number {
  const full = (1 << skills) - 1;
  const moves = [...new Set(people)];
  const dist = new Int32Array(full + 1).fill(-1);
  dist[0] = 0;
  const queue = [0];
  for (let head = 0; head < queue.length; head++) {
    const mask = queue[head];
    if (mask === full) return dist[mask];
    for (const p of moves) {
      const m = mask | p;
      if (dist[m] < 0) {
        dist[m] = dist[mask] + 1;
        queue.push(m);
      }
    }
  }
  return -1;
}
