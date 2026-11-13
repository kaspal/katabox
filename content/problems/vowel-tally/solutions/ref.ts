export function vowelTally(text: string): number {
  let count = 0;
  for (const c of text) {
    if ("aeiou".includes(c.toLowerCase())) count++;
  }
  return count;
}
