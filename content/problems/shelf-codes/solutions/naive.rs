pub fn count_below(codes: Vec<i64>, queries: Vec<i64>) -> Vec<i64> {
    // Correct but O(n·q): count the smaller numbers for every query.
    queries
        .iter()
        .map(|&x| codes.iter().filter(|&&c| c < x).count() as i64)
        .collect()
}
