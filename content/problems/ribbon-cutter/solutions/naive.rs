pub fn longest_piece(ribbons: Vec<i64>, pieces: i64) -> i64 {
    // Correct but O(n·M): try lengths from the longest roll downwards.
    let mut length = *ribbons.iter().max().unwrap();
    while length >= 1 {
        let count: i64 = ribbons.iter().map(|r| r / length).sum();
        if count >= pieces {
            return length;
        }
        length -= 1;
    }
    0
}
