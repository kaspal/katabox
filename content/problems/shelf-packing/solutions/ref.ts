export function fewestShelves(widths: number[], width: number): number {
  const books = [...widths].sort((a, b) => b - a);
  let total = 0;
  for (const b of books) total += b;
  const lower = Math.ceil(total / width);
  const ffd: number[] = [];
  for (const b of books) {
    const k = ffd.findIndex((load) => load + b <= width);
    if (k >= 0) ffd[k] += b;
    else ffd.push(b);
  }
  let best = ffd.length;
  const loads: number[] = [];
  const place = (i: number): void => {
    if (best === lower) return;
    if (i === books.length) {
      best = Math.min(best, loads.length);
      return;
    }
    const b = books[i];
    const tried = new Set<number>();
    for (let k = 0; k < loads.length; k++) {
      if (loads[k] + b <= width && !tried.has(loads[k])) {
        tried.add(loads[k]);
        loads[k] += b;
        place(i + 1);
        loads[k] -= b;
      }
    }
    if (loads.length + 1 < best) {
      loads.push(b);
      place(i + 1);
      loads.pop();
    }
  };
  place(0);
  return best;
}
