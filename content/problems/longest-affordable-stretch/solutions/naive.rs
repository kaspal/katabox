pub fn longest_stretch(costs: Vec<i64>, budget: i64) -> i64 {
    // Correct but O(n²) when the budget is large: every start is extended to the end.
    let mut best = 0;
    let n = costs.len();
    for i in 0..n {
        let mut total = 0;
        for j in i..n {
            total += costs[j];
            if total > budget {
                break;
            }
            best = best.max(j - i + 1);
        }
    }
    best as i64
}
