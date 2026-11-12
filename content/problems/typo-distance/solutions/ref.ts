export function typoDistance(typed: string, meant: string): number {
  const m = meant.length;
  let prev = new Int32Array(m + 1);
  let cur = new Int32Array(m + 1);
  for (let j = 0; j <= m; j++) prev[j] = j;
  for (let i = 1; i <= typed.length; i++) {
    cur[0] = i;
    const x = typed.charCodeAt(i - 1);
    for (let j = 1; j <= m; j++) {
      if (x === meant.charCodeAt(j - 1)) cur[j] = prev[j - 1];
      else cur[j] = 1 + Math.min(prev[j], cur[j - 1], prev[j - 1]);
    }
    [prev, cur] = [cur, prev];
  }
  return prev[m];
}
