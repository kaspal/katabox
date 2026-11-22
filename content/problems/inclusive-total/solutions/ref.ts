export function rangeTotal(nums: number[], a: number, b: number): number {
  let total = 0;
  for (let i = a; i <= b; i++) total += nums[i];
  return total;
}
