use std::collections::HashMap;

pub fn two_pivots(weights: Vec<i64>) -> i64 {
    let n = weights.len();
    let mut prefix = vec![0i64; n + 1];
    for k in 0..n {
        prefix[k + 1] = prefix[k] + weights[k];
    }
    let total = prefix[n];
    let mut seen: HashMap<(i64, i64), i64> = HashMap::new();
    let mut count = 0;
    for j in 1..n {
        let i = j - 1;
        *seen.entry((prefix[i], weights[i])).or_insert(0) += 1;
        let right = total - prefix[j + 1];
        count += seen.get(&(right, prefix[j] - 2 * right)).copied().unwrap_or(0);
    }
    count
}
