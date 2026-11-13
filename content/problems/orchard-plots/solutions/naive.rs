pub fn plot_totals(grid: Vec<Vec<i64>>, queries: Vec<Vec<i64>>) -> Vec<i64> {
    // Correct but O(rows · columns) per query: sum each plot row by row.
    queries
        .iter()
        .map(|q| {
            let (t, l, b, r) = (q[0] as usize, q[1] as usize, q[2] as usize, q[3] as usize);
            grid[t..=b].iter().map(|row| row[l..=r].iter().sum::<i64>()).sum()
        })
        .collect()
}
