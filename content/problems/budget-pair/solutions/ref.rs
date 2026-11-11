pub fn best_pair(prices: Vec<i64>, budget: i64) -> i64 {
    let (mut lo, mut hi) = (0usize, prices.len() - 1);
    let mut best = -1;
    while lo < hi {
        let total = prices[lo] + prices[hi];
        if total > budget {
            hi -= 1;
        } else {
            best = best.max(total);
            lo += 1;
        }
    }
    best
}
