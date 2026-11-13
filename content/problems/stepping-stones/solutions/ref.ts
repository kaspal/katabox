export function canCross(jumps: number[]): boolean {
  let reach = 0;
  for (let i = 0; i < jumps.length; i++) {
    if (i > reach) return false;
    reach = Math.max(reach, i + jumps[i]);
  }
  return true;
}
