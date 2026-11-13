pub fn digit_budget(limit: i64, total: i64) -> i64 {
    // Correct but O(limit): check every ticket.
    let mut count = 0;
    for t in 1..=limit {
        let (mut s, mut x) = (0, t);
        while x > 0 {
            s += x % 10;
            x /= 10;
        }
        if s == total {
            count += 1;
        }
    }
    count
}
