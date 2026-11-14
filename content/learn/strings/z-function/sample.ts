function zSearch(text: string, pattern: string): number[] {
  const s = pattern + "|" + text;
  const n = s.length;
  const z = new Array<number>(n).fill(0);
  let left = 0;
  let right = 0;
  for (let i = 1; i < n; i++) {
    if (i < right) {
      z[i] = Math.min(right - i, z[i - left]); // @step reuse
    }
    while (i + z[i] < n && s[z[i]] === s[i + z[i]]) {
      z[i]++; // @step extend
    }
    if (i + z[i] > right) {
      left = i;
      right = i + z[i];
    }
  }
  const m = pattern.length;
  const hits: number[] = [];
  for (let i = m + 1; i < n; i++) if (z[i] === m) hits.push(i - m - 1);
  return [hits.length, ...hits];
}
