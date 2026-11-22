export function totalCovered(shifts: number[][]): number {
  const sorted = [...shifts].sort((x, y) => x[0] - y[0] || x[1] - y[1]);
  let total = 0;
  let curS = sorted[0][0];
  let curE = sorted[0][1];
  for (const [s, e] of sorted.slice(1)) {
    if (s <= curE) {
      curE = Math.max(curE, e);
    } else {
      total += curE - curS;
      curS = s;
      curE = e;
    }
  }
  return total + curE - curS;
}
