pub fn plot_totals(grid: Vec<Vec<i64>>, queries: Vec<Vec<i64>>) -> Vec<i64> {
    let cols = grid[0].len();
    let w = cols + 1;
    let mut pre = vec![0i64; (grid.len() + 1) * w];
    for (r, row) in grid.iter().enumerate() {
        let mut run = 0;
        for c in 0..cols {
            run += row[c];
            pre[(r + 1) * w + c + 1] = pre[r * w + c + 1] + run;
        }
    }
    queries
        .iter()
        .map(|q| {
            let (t, l, b, r) = (q[0] as usize, q[1] as usize, q[2] as usize + 1, q[3] as usize + 1);
            pre[b * w + r] - pre[t * w + r] - pre[b * w + l] + pre[t * w + l]
        })
        .collect()
}
