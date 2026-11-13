pub fn longest_repeat(text: String) -> i64 {
    // Correct but O(n² · L): compare every pair of starting positions letter by letter.
    let s = text.as_bytes();
    let n = s.len();
    let mut best = 0;
    for i in 0..n {
        for j in i + 1..n {
            let mut k = 0;
            while j + k < n && s[i + k] == s[j + k] {
                k += 1;
            }
            best = best.max(k);
        }
    }
    best as i64
}
