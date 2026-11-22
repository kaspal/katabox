export function countPairs(nums: number[], target: number): number {
  const seen = new Map<number, number>();
  let pairs = 0;
  for (const n of nums) {
    pairs += seen.get(target - n) ?? 0;
    seen.set(n, (seen.get(n) ?? 0) + 1);
  }
  return pairs;
}
