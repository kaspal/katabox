export function mostVowels(words: string[]): number {
  let best = 0;
  for (const w of words) {
    let count = 0;
    for (const c of w) if ("aeiou".includes(c)) count++;
    best = Math.max(best, count);
  }
  return best;
}
