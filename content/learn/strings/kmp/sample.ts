function kmp(text: string, pattern: string): number[] {
  const m = pattern.length;
  const fail = new Array<number>(m).fill(0);
  let k = 0;
  for (let i = 1; i < m; i++) {
    while (k > 0 && pattern[i] !== pattern[k]) {
      k = fail[k - 1]; // @step fall
    }
    if (pattern[i] === pattern[k]) k++;
    fail[i] = k; // @step border
  }
  const hits: number[] = [];
  k = 0;
  for (let i = 0; i < text.length; i++) {
    while (k > 0 && text[i] !== pattern[k]) {
      k = fail[k - 1]; // @step skip
    }
    if (text[i] === pattern[k]) k++;
    if (k === m) {
      hits.push(i - m + 1); // @step match
      k = fail[k - 1];
    }
  }
  return [hits.length, ...hits];
}
