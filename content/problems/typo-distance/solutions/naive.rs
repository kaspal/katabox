pub fn typo_distance(typed: String, meant: String) -> i64 {
    // Correct but exponential: plain recursion on the remaining suffixes.
    fn go(a: &[u8], b: &[u8], i: usize, j: usize) -> i64 {
        if i == a.len() {
            return (b.len() - j) as i64;
        }
        if j == b.len() {
            return (a.len() - i) as i64;
        }
        if a[i] == b[j] {
            return go(a, b, i + 1, j + 1);
        }
        1 + go(a, b, i + 1, j)
            .min(go(a, b, i, j + 1))
            .min(go(a, b, i + 1, j + 1))
    }
    go(typed.as_bytes(), meant.as_bytes(), 0, 0)
}
