fn go(t: &[u8], start: usize) -> i64 {
    if start == t.len() {
        return -1;
    }
    let mut best = t.len() as i64;
    for end in start + 1..=t.len() {
        let piece = &t[start..end];
        if piece.iter().eq(piece.iter().rev()) {
            best = best.min(1 + go(t, end));
        }
    }
    best
}

pub fn mirror_cuts(text: String) -> i64 {
    // Correct but exponential: try every first piece recursively, remembering nothing.
    go(text.as_bytes(), 0)
}
