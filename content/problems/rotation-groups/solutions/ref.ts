export function rotationGroups(codes: string[]): number {
  const seen = new Set<string>();
  for (const code of codes) {
    let best = code;
    for (let i = 1; i < code.length; i++) {
      const r = code.slice(i) + code.slice(0, i);
      if (r < best) best = r;
    }
    seen.add(best);
  }
  return seen.size;
}
