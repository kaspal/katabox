export function shortestWalk(stalls: string, need: string): number {
  const want = new Int32Array(26);
  for (let k = 0; k < need.length; k++) want[need.charCodeAt(k) - 97]++;
  const have = new Int32Array(26);
  let missing = need.length;
  let left = 0;
  let best = -1;
  for (let right = 0; right < stalls.length; right++) {
    const c = stalls.charCodeAt(right) - 97;
    if (have[c] < want[c]) missing--;
    have[c]++;
    while (missing === 0) {
      if (best < 0 || right - left + 1 < best) best = right - left + 1;
      const d = stalls.charCodeAt(left) - 97;
      have[d]--;
      if (have[d] < want[d]) missing++;
      left++;
    }
  }
  return best;
}
