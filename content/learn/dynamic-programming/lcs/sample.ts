function lcs(a: string, b: string): number[] {
  const m = a.length;
  const n = b.length;
  const best = Array.from({ length: m + 1 }, () =>
    new Array<number>(n + 1).fill(0),
  );
  for (let i = 1; i <= m; i++) {
    for (let j = 1; j <= n; j++) {
      if (a[i - 1] === b[j - 1]) {
        best[i][j] = best[i - 1][j - 1] + 1; // @step match
      } else {
        best[i][j] = Math.max(best[i - 1][j], best[i][j - 1]); // @step carry
      }
    }
  }
  const out: string[] = [];
  let i = m;
  let j = n;
  while (i > 0 && j > 0) {
    if (a[i - 1] === b[j - 1]) {
      out.push(a[i - 1]); // @step trace
      i--;
      j--;
    } else if (best[i - 1][j] >= best[i][j - 1]) {
      i--;
    } else {
      j--;
    }
  }
  return [out.length, ...out.reverse().map((c) => c.charCodeAt(0) - 97)];
}
