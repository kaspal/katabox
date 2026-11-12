pub fn tilings(n: i64) -> i64 {
    // Correct but exponential: walk through every paving (an explicit stack).
    let mut count = 0i64;
    let mut stack = vec![n];
    while let Some(left) = stack.pop() {
        if left == 0 {
            count = (count + 1) % 1_000_000_007;
            continue;
        }
        for p in 1..=3 {
            if p <= left {
                stack.push(left - p);
            }
        }
    }
    count
}
