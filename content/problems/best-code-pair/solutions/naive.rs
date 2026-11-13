pub fn best_xor_pair(codes: Vec<i64>) -> i64 {
    // Correct but O(n²): try every pair.
    let mut best = 0;
    for i in 0..codes.len() {
        for j in i + 1..codes.len() {
            best = best.max(codes[i] ^ codes[j]);
        }
    }
    best
}
