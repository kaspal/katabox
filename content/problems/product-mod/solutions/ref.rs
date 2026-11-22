pub fn product_mod(nums: Vec<i64>) -> i64 {
    const MOD: i64 = 1_000_000_007;
    nums.iter().fold(1, |acc, &x| acc * (x % MOD) % MOD)
}
