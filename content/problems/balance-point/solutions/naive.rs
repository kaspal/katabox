pub fn balance_point(weights: Vec<i64>) -> i64 {
    // Correct but O(n²): both sides summed from scratch for every index.
    let n = weights.len();
    for i in 0..n {
        let left: i64 = weights[..i].iter().sum();
        let right: i64 = weights[i + 1..].iter().sum();
        if left == right {
            return i as i64;
        }
    }
    -1
}
