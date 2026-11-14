function wordBreak(text: string, words: string[]): number[] {
  const n = text.length;
  const ok = new Array<boolean>(n + 1).fill(false);
  ok[0] = true;
  for (let end = 1; end <= n; end++) {
    for (const word of words) {
      const start = end - word.length;
      if (start >= 0 && ok[start] && text.slice(start, end) === word) {
        ok[end] = true; // @step fits
        break;
      }
    }
  }
  return [ok[n] ? 1 : 0];
}
