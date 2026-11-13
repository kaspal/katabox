export function longestMirror(text: string): string {
  const n = text.length;
  let bestStart = 0;
  let bestLen = 0;
  // Middles in order: letter c (odd) then the gap after it (even).
  for (let c = 0; c < n; c++) {
    for (const [lo0, hi0] of [[c, c], [c, c + 1]]) {
      let lo = lo0;
      let hi = hi0;
      while (lo >= 0 && hi < n && text[lo] === text[hi]) {
        lo--;
        hi++;
      }
      const len = hi - lo - 1;
      if (len > bestLen) {
        bestLen = len;
        bestStart = lo + 1;
      }
    }
  }
  return text.slice(bestStart, bestStart + bestLen);
}
