pub fn biggest_tank(boards: Vec<i64>) -> i64 {
    let (mut i, mut j, mut best) = (0, boards.len() - 1, 0);
    while i < j {
        let width = (j - i) as i64;
        if boards[i] <= boards[j] {
            best = best.max(width * boards[i]);
            i += 1;
        } else {
            best = best.max(width * boards[j]);
            j -= 1;
        }
    }
    best
}
