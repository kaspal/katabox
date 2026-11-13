pub fn count_pattern(text: String, pattern: String) -> i64 {
    // Correct but O(n · m): compare character by character at every position.
    let (t, p) = (text.as_bytes(), pattern.as_bytes());
    let (n, m) = (t.len(), p.len());
    let mut count = 0;
    if m > n {
        return 0;
    }
    for i in 0..=n - m {
        let mut k = 0;
        while k < m && t[i + k] == p[k] {
            k += 1;
        }
        if k == m {
            count += 1;
        }
    }
    count
}
