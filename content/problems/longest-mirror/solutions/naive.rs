pub fn longest_mirror(text: String) -> String {
    // Correct but O(n³): check every piece letter by letter.
    let s = text.as_bytes();
    let n = s.len();
    let (mut best_start, mut best_len) = (0, 0);
    for i in 0..n {
        for j in i..n {
            let (mut lo, mut hi) = (i, j);
            while lo < hi && s[lo] == s[hi] {
                lo += 1;
                hi -= 1;
            }
            if lo >= hi && j - i + 1 > best_len {
                best_start = i;
                best_len = j - i + 1;
            }
        }
    }
    text[best_start..best_start + best_len].to_string()
}
