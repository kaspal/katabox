const MOD = 1000000007n;

export function productMod(nums: number[]): number {
  // acc * x can reach 10^18, beyond 2^53: multiply as BigInt.
  let acc = 1n;
  for (const x of nums) acc = (acc * BigInt(x)) % MOD;
  return Number(acc);
}
