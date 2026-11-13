export function mirrorCuts(text: string): number {
  const n = text.length;
  const cuts = Array.from({ length: n + 1 }, (_, i) => i - 1);
  for (let c = 0; c < n; c++) {
    for (const [l0, r0] of [[c, c], [c, c + 1]]) {
      let l = l0;
      let r = r0;
      while (l >= 0 && r < n && text[l] === text[r]) {
        if (cuts[l] + 1 < cuts[r + 1]) cuts[r + 1] = cuts[l] + 1;
        l--;
        r++;
      }
    }
  }
  return cuts[n];
}
