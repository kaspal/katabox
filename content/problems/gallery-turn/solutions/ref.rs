pub fn turn_right(grid: Vec<Vec<i64>>) -> Vec<Vec<i64>> {
    let (r, c) = (grid.len(), grid[0].len());
    (0..c)
        .map(|j| (0..r).map(|i| grid[r - 1 - i][j]).collect())
        .collect()
}
