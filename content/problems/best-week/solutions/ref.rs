pub fn best_week(sales: Vec<i64>, k: i64) -> i64 {
    let k = k as usize;
    let mut window: i64 = sales[..k].iter().sum();
    let mut best = window;
    for i in k..sales.len() {
        window += sales[i] - sales[i - k];
        best = best.max(window);
    }
    best
}
