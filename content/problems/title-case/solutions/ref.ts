export function titleCase(text: string): string {
  let out = "";
  let atStart = true;
  for (const c of text) {
    if (c === " ") {
      out += c;
      atStart = true;
    } else {
      out += atStart ? c.toUpperCase() : c.toLowerCase();
      atStart = false;
    }
  }
  return out;
}
