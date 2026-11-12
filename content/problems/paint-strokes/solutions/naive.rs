pub fn painted_length(strokes: Vec<Vec<i64>>) -> i64 {
    // Correct but O(n²) when strokes do not overlap: merge each stroke into a list of
    // disjoint runs by scanning the whole list.
    let mut runs: Vec<(i64, i64)> = Vec::new();
    for s in &strokes {
        let (mut start, mut end) = (s[0], s[1]);
        let mut kept = Vec::with_capacity(runs.len() + 1);
        for &(a, b) in &runs {
            if b < start || end < a {
                kept.push((a, b));
            } else {
                start = start.min(a);
                end = end.max(b);
            }
        }
        kept.push((start, end));
        runs = kept;
    }
    runs.iter().map(|(a, b)| b - a).sum()
}
