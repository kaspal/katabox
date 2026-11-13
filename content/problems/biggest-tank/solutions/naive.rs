pub fn biggest_tank(boards: Vec<i64>) -> i64 {
    // Correct but O(n²): every pair of boards.
    let mut best = 0;
    for i in 0..boards.len() {
        for j in i + 1..boards.len() {
            best = best.max((j - i) as i64 * boards[i].min(boards[j]));
        }
    }
    best
}
