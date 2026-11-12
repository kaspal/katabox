pub fn balance_point(weights: Vec<i64>) -> i64 {
    let total: i64 = weights.iter().sum();
    let mut left = 0;
    for (i, w) in weights.iter().enumerate() {
        if left == total - left - w {
            return i as i64;
        }
        left += w;
    }
    -1
}
