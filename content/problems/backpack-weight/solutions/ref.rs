pub fn best_pack(weights: Vec<i64>, values: Vec<i64>, capacity: i64) -> i64 {
    let cap = capacity as usize;
    let mut best = vec![0i64; cap + 1];
    for (&w, &v) in weights.iter().zip(&values) {
        let w = w as usize;
        for c in (w..=cap).rev() {
            best[c] = best[c].max(best[c - w] + v);
        }
    }
    best[cap]
}
