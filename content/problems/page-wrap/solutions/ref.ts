export function wrapLines(words: string[], width: number): string[] {
  const lines: string[] = [];
  let current = [words[0]];
  let length = words[0].length;
  for (let i = 1; i < words.length; i++) {
    const w = words[i];
    if (length + 1 + w.length <= width) {
      current.push(w);
      length += 1 + w.length;
    } else {
      lines.push(current.join(" "));
      current = [w];
      length = w.length;
    }
  }
  lines.push(current.join(" "));
  return lines;
}
