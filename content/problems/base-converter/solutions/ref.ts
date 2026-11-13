const DIGITS = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";

export function convertBase(number: string, source: number, target: number): string {
  let value = 0;
  for (const ch of number) value = value * source + DIGITS.indexOf(ch);
  if (value === 0) return "0";
  const out: string[] = [];
  while (value > 0) {
    out.push(DIGITS[value % target]);
    value = Math.floor(value / target);
  }
  return out.reverse().join("");
}
