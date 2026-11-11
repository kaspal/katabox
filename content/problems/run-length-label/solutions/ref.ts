export function compress(text: string): string {
  const out: string[] = [];
  let start = 0;
  for (let i = 1; i <= text.length; i++) {
    if (i === text.length || text[i] !== text[start]) {
      out.push(text[start]);
      if (i - start > 1) out.push(String(i - start));
      start = i;
    }
  }
  return out.join("");
}
