export function withoutBanned(items: number[], banned: number[]): number[] {
  const bad = new Set(banned);
  return items.filter((x) => !bad.has(x));
}
