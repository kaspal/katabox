pub fn plantings(length: i64, widths: Vec<i64>) -> i64 {
    // Correct but exponential: walk through every planting (with an explicit stack).
    let mut count = 0i64;
    let mut stack = vec![(length, usize::MAX)];
    while let Some((left, last)) = stack.pop() {
        if left == 0 {
            count = (count + 1) % 1_000_000_007;
            continue;
        }
        for (k, &w) in widths.iter().enumerate() {
            if k != last && w <= left {
                stack.push((left - w, k));
            }
        }
    }
    count
}
