use std::collections::HashMap;

pub fn two_sum(nums: Vec<i64>, target: i64) -> Vec<usize> {
    let mut seen: HashMap<i64, usize> = HashMap::new();
    for (i, &n) in nums.iter().enumerate() {
        if let Some(&j) = seen.get(&(target - n)) {
            return vec![j, i];
        }
        seen.insert(n, i);
    }
    Vec::new()
}
