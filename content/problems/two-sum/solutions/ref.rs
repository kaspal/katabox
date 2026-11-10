use std::collections::HashMap;

pub fn two_sum(nums: Vec<i64>, target: i64) -> Vec<i64> {
    let mut seen: HashMap<i64, i64> = HashMap::new();
    for (i, &n) in nums.iter().enumerate() {
        let i = i as i64;
        if let Some(&j) = seen.get(&(target - n)) {
            return vec![j, i];
        }
        seen.insert(n, i);
    }
    Vec::new()
}
