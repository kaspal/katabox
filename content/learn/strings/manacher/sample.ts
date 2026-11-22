function manacher(text: string): number[] {
  const s = "|" + text.split("").join("|") + "|";
  const n = s.length;
  const radius = new Array<number>(n).fill(0);
  let center = 0;
  let right = 0;
  for (let i = 0; i < n; i++) {
    if (i < right) {
      radius[i] = Math.min(right - i, radius[2 * center - i]); // @step mirror
    }
    while (
      i - radius[i] - 1 >= 0 &&
      i + radius[i] + 1 < n &&
      s[i - radius[i] - 1] === s[i + radius[i] + 1]
    ) {
      radius[i]++; // @step grow
    }
    if (i + radius[i] > right) {
      center = i;
      right = i + radius[i];
    }
  }
  let best = 0;
  for (let i = 0; i < n; i++) if (radius[i] > radius[best]) best = i;
  return [radius[best], (best - radius[best]) / 2];
}
