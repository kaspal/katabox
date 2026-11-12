pub fn balanced_stretches(results: String) -> i64 {
    let n = results.len();
    let mut seen = vec![0i64; 2 * n + 1];
    let mut score = n; // the score shifted by n, so it never goes below 0
    seen[score] = 1;
    let mut count = 0i64;
    for b in results.bytes() {
        match b {
            b'W' => score += 1,
            b'L' => score -= 1,
            _ => {}
        }
        count += seen[score];
        seen[score] += 1;
    }
    count
}
