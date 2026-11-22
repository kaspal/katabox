const MOD = 1000000007;

export function productMod(nums: number[]): number {
  let acc = 1;
  for (const x of nums) acc = (acc * x) % MOD;
  return acc;
}
