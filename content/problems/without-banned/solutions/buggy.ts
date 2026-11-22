export function withoutBanned(items: number[], banned: number[]): number[] {
  const bad = new Set(banned);
  for (let i = 0; i < items.length; i++) {
    if (bad.has(items[i])) items.splice(i, 1);
  }
  return items;
}
