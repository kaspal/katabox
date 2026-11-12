pub fn range_totals(rain: Vec<i64>, queries: Vec<Vec<i64>>) -> Vec<i64> {
    // Correct but O(n·q): each query re-sums its range.
    queries
        .iter()
        .map(|q| rain[q[0] as usize..=q[1] as usize].iter().sum())
        .collect()
}
