pub fn first_at_least(sorted: Vec<i64>, x: i64) -> i64 {
    let (mut lo, mut hi) = (0usize, sorted.len() - 1);
    while lo < hi {
        let mid = (lo + hi) / 2;
        if sorted[mid] < x {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    lo as i64
}
