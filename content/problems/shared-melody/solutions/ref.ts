export function sharedMelody(first: number[], second: number[]): number {
  const m = second.length;
  let prev = new Int32Array(m + 1);
  let cur = new Int32Array(m + 1);
  for (const x of first) {
    for (let j = 1; j <= m; j++) {
      cur[j] = x === second[j - 1] ? prev[j - 1] + 1 : Math.max(prev[j], cur[j - 1]);
    }
    [prev, cur] = [cur, prev];
  }
  return prev[m];
}
