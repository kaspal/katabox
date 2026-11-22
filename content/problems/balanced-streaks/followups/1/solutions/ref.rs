pub fn longest_balanced(results: String) -> i64 {
    let n = results.len();
    // first[score + n]: the earliest prefix with that score (-2: never seen).
    let mut first = vec![-2i64; 2 * n + 1];
    let mut score = n;
    first[score] = -1;
    let mut best = 0;
    for (i, b) in results.bytes().enumerate() {
        match b {
            b'W' => score += 1,
            b'L' => score -= 1,
            _ => {}
        }
        if first[score] == -2 {
            first[score] = i as i64;
        } else {
            best = best.max(i as i64 - first[score]);
        }
    }
    best
}
