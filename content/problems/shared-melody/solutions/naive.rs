fn go(a: &[i64], b: &[i64], i: usize, j: usize) -> i64 {
    if i == a.len() || j == b.len() {
        return 0;
    }
    if a[i] == b[j] {
        return 1 + go(a, b, i + 1, j + 1);
    }
    go(a, b, i + 1, j).max(go(a, b, i, j + 1))
}

pub fn shared_melody(first: Vec<i64>, second: Vec<i64>) -> i64 {
    // Correct but exponential: recurse on both choices without remembering results.
    go(&first, &second, 0, 0)
}
