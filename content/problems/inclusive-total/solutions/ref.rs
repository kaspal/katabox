pub fn range_total(nums: Vec<i64>, a: i64, b: i64) -> i64 {
    nums[a as usize..=b as usize].iter().sum()
}
