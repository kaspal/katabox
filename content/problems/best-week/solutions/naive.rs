pub fn best_week(sales: Vec<i64>, k: i64) -> i64 {
    // Correct but O(n·k): every window summed from scratch.
    let k = k as usize;
    let mut best = 0;
    for start in 0..=sales.len() - k {
        let total: i64 = sales[start..start + k].iter().sum();
        best = best.max(total);
    }
    best
}
