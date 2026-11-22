pub fn fleet_totals(miles: Vec<Vec<i64>>, queries: Vec<Vec<i64>>) -> Vec<i64> {
    let (rows, cols) = (miles.len(), miles[0].len());
    let mut p = vec![vec![0i64; cols + 1]; rows + 1];
    for r in 0..rows {
        for c in 0..cols {
            p[r + 1][c + 1] = miles[r][c] + p[r][c + 1] + p[r + 1][c] - p[r][c];
        }
    }
    queries
        .iter()
        .map(|q| {
            let (v1, d1, v2, d2) = (q[0] as usize, q[1] as usize, q[2] as usize, q[3] as usize);
            p[v2 + 1][d2 + 1] - p[v1][d2 + 1] - p[v2 + 1][d1] + p[v1][d1]
        })
        .collect()
}
