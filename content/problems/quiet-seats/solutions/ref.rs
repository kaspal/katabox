pub fn quiet_seats(rows: Vec<String>) -> i64 {
    let grid: Vec<&[u8]> = rows.iter().map(|r| r.as_bytes()).collect();
    let (h, w) = (grid.len(), grid[0].len());
    let mut count = 0;
    for r in 0..h {
        for c in 0..w {
            if grid[r][c] != b'.' {
                continue;
            }
            let quiet = (r.saturating_sub(1)..=(r + 1).min(h - 1)).all(|i| {
                (c.saturating_sub(1)..=(c + 1).min(w - 1)).all(|j| grid[i][j] != b'#')
            });
            if quiet {
                count += 1;
            }
        }
    }
    count
}
