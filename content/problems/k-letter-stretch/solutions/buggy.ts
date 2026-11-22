export function longestWithKLetters(text: string, k: number): number {
  const count = new Map<string, number>();
  let best = 0;
  let start = 0;
  for (let end = 0; end < text.length; end++) {
    count.set(text[end], (count.get(text[end]) ?? 0) + 1);
    best = Math.max(best, end - start + 1);
    while (count.size > k) {
      const s = text[start++];
      const c = count.get(s)! - 1;
      if (c === 0) count.delete(s);
      else count.set(s, c);
    }
  }
  return best;
}
