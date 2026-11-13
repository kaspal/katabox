export function twoTeams(n: number, rivalries: number[][]): boolean {
  const adj: number[][] = Array.from({ length: n }, () => []);
  for (const [a, b] of rivalries) {
    adj[a].push(b);
    adj[b].push(a);
  }
  const colour = new Int8Array(n).fill(-1);
  const queue = new Int32Array(n);
  for (let s = 0; s < n; s++) {
    if (colour[s] !== -1) continue;
    colour[s] = 0;
    let head = 0;
    let tail = 0;
    queue[tail++] = s;
    while (head < tail) {
      const x = queue[head++];
      for (const y of adj[x]) {
        if (colour[y] === -1) {
          colour[y] = 1 - colour[x];
          queue[tail++] = y;
        } else if (colour[y] === colour[x]) {
          return false;
        }
      }
    }
  }
  return true;
}
