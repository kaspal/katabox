function solveQueens(n: number): number[] {
  const cols: number[] = [];
  const found: number[][] = [];
  const safe = (c: number): boolean => {
    const r = cols.length;
    return cols.every((cc, rr) => c !== cc && Math.abs(c - cc) !== r - rr);
  };
  const place = (): void => {
    if (cols.length === n) {
      found.push([...cols]); // @step solution
      return;
    }
    for (let c = 0; c < n; c++) {
      if (safe(c)) {
        cols.push(c); // @step place
        place();
        cols.pop(); // @step remove
      }
    }
  };
  place();
  return [found.length, ...(found.length > 0 ? found[0] : [])];
}
