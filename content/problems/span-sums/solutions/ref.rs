pub fn span_sums(rain: Vec<i64>, queries: Vec<Vec<i64>>) -> Vec<i64> {
    let mut prefix = vec![0i64];
    for x in &rain {
        prefix.push(prefix[prefix.len() - 1] + x);
    }
    queries
        .iter()
        .map(|q| prefix[q[1] as usize + 1] - prefix[q[0] as usize])
        .collect()
}
