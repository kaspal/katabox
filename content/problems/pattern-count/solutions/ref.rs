pub fn count_pattern(text: String, pattern: String) -> i64 {
    let (t, p) = (text.as_bytes(), pattern.as_bytes());
    let m = p.len();
    let mut pi = vec![0usize; m];
    let mut k = 0;
    for i in 1..m {
        while k > 0 && p[i] != p[k] {
            k = pi[k - 1];
        }
        if p[i] == p[k] {
            k += 1;
        }
        pi[i] = k;
    }
    let mut count = 0;
    k = 0;
    for &c in t {
        while k > 0 && c != p[k] {
            k = pi[k - 1];
        }
        if c == p[k] {
            k += 1;
        }
        if k == m {
            count += 1;
            k = pi[k - 1];
        }
    }
    count
}
