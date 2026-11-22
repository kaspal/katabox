const INF = 1e6;

function tsp(n: number, dist: number[][]): number[] {
  const full = 1 << n;
  const best = Array.from({ length: full }, () =>
    new Array<number>(n).fill(INF),
  );
  best[1][0] = 0;
  for (let mask = 1; mask < full; mask += 2) {
    for (let last = 0; last < n; last++) {
      if (best[mask][last] === INF) continue;
      for (let city = 0; city < n; city++) {
        if (mask & (1 << city)) continue;
        const next = mask | (1 << city);
        const total = best[mask][last] + dist[last][city];
        if (total < best[next][city]) {
          best[next][city] = total; // @step extend
        }
      }
    }
  }
  let answer = INF;
  for (let last = 1; last < n; last++) {
    answer = Math.min(answer, best[full - 1][last] + dist[last][0]);
  }
  return [answer];
}
