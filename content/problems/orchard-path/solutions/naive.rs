pub fn most_apples(grid: Vec<Vec<i64>>) -> i64 {
    // Correct but exponential: explore every path recursively, with no memory of
    // trees already solved.
    fn go(grid: &[Vec<i64>], r: usize, c: usize) -> i64 {
        let (rows, cols) = (grid.len(), grid[0].len());
        let here = grid[r][c];
        if r == rows - 1 && c == cols - 1 {
            return here;
        }
        let mut best = -1;
        if r + 1 < rows {
            best = best.max(go(grid, r + 1, c));
        }
        if c + 1 < cols {
            best = best.max(go(grid, r, c + 1));
        }
        here + best
    }
    go(&grid, 0, 0)
}
