pub fn range_totals(rain: Vec<i64>, queries: Vec<Vec<i64>>) -> Vec<i64> {
    let mut prefix = vec![0i64; rain.len() + 1];
    for (d, r) in rain.iter().enumerate() {
        prefix[d + 1] = prefix[d] + r;
    }
    queries
        .iter()
        .map(|q| prefix[q[1] as usize + 1] - prefix[q[0] as usize])
        .collect()
}
