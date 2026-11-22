use std::collections::HashMap;

pub fn count_pairs(nums: Vec<i64>, target: i64) -> i64 {
    let mut seen: HashMap<i64, i64> = HashMap::new();
    let mut pairs = 0;
    for n in nums {
        pairs += seen.get(&(target - n)).copied().unwrap_or(0);
        *seen.entry(n).or_insert(0) += 1;
    }
    pairs
}
