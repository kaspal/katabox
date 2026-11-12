pub fn most_apples(grid: Vec<Vec<i64>>) -> i64 {
    let cols = grid[0].len();
    let mut best = vec![0i64; cols];
    for (r, row) in grid.iter().enumerate() {
        for c in 0..cols {
            let prev = if r == 0 {
                if c > 0 { best[c - 1] } else { 0 }
            } else if c == 0 {
                best[0]
            } else {
                best[c].max(best[c - 1])
            };
            best[c] = row[c] + prev;
        }
    }
    best[cols - 1]
}
