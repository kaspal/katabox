pub fn longest_stretch(costs: Vec<i64>, budget: i64) -> i64 {
    let mut best = 0i64;
    let mut left = 0usize;
    let mut total = 0i64;
    for right in 0..costs.len() {
        total += costs[right];
        while total > budget {
            total -= costs[left];
            left += 1;
        }
        best = best.max(right as i64 - left as i64 + 1);
    }
    best
}
