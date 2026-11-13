pub fn longest_mirror(text: String) -> String {
    let s = text.as_bytes();
    let n = s.len() as i64;
    let (mut best_start, mut best_len) = (0i64, 0i64);
    // Middles in order: letter c (odd) then the gap after it (even).
    for c in 0..n {
        for (mut lo, mut hi) in [(c, c), (c, c + 1)] {
            while lo >= 0 && hi < n && s[lo as usize] == s[hi as usize] {
                lo -= 1;
                hi += 1;
            }
            let len = hi - lo - 1;
            if len > best_len {
                best_len = len;
                best_start = lo + 1;
            }
        }
    }
    text[best_start as usize..(best_start + best_len) as usize].to_string()
}
