export function closestSum(nums: number[], target: number): number {
  const a = [...nums].sort((x, y) => x - y);
  let i = 0;
  let j = a.length - 1;
  let best = a[0] + a[1];
  while (i < j) {
    const s = a[i] + a[j];
    const d = Math.abs(s - target);
    const bd = Math.abs(best - target);
    if (d < bd || (d === bd && s < best)) best = s;
    if (s < target) i++;
    else if (s > target) j--;
    else return s;
  }
  return best;
}
