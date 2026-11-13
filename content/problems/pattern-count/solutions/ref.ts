export function countPattern(text: string, pattern: string): number {
  const m = pattern.length;
  const pi = new Int32Array(m);
  let k = 0;
  for (let i = 1; i < m; i++) {
    while (k > 0 && pattern[i] !== pattern[k]) k = pi[k - 1];
    if (pattern[i] === pattern[k]) k++;
    pi[i] = k;
  }
  let count = 0;
  k = 0;
  for (let i = 0; i < text.length; i++) {
    const c = text[i];
    while (k > 0 && c !== pattern[k]) k = pi[k - 1];
    if (c === pattern[k]) k++;
    if (k === m) {
      count++;
      k = pi[k - 1];
    }
  }
  return count;
}
