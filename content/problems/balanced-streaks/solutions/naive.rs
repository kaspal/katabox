pub fn balanced_stretches(results: String) -> i64 {
    // Correct but O(n²): extend every run one match at a time.
    let b = results.as_bytes();
    let mut count = 0i64;
    for i in 0..b.len() {
        let mut score = 0i64;
        for &c in &b[i..] {
            match c {
                b'W' => score += 1,
                b'L' => score -= 1,
                _ => {}
            }
            if score == 0 {
                count += 1;
            }
        }
    }
    count
}
