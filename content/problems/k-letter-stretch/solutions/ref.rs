pub fn longest_with_k_letters(text: String, k: i64) -> i64 {
    let t = text.as_bytes();
    let mut count = [0i64; 26];
    let (mut distinct, mut start, mut best) = (0i64, 0usize, 0usize);
    for end in 0..t.len() {
        let c = (t[end] - b'a') as usize;
        if count[c] == 0 {
            distinct += 1;
        }
        count[c] += 1;
        while distinct > k {
            let s = (t[start] - b'a') as usize;
            count[s] -= 1;
            if count[s] == 0 {
                distinct -= 1;
            }
            start += 1;
        }
        best = best.max(end + 1 - start);
    }
    best as i64
}
