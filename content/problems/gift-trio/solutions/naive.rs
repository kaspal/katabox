pub fn closest_trio(prices: Vec<i64>, target: i64) -> i64 {
    // Correct but O(n³): every trio.
    let n = prices.len();
    let mut best = prices[0] + prices[1] + prices[2];
    for i in 0..n {
        for j in i + 1..n {
            for k in j + 1..n {
                let s = prices[i] + prices[j] + prices[k];
                let (g, bg) = ((s - target).abs(), (best - target).abs());
                if g < bg || (g == bg && s < best) {
                    best = s;
                }
            }
        }
    }
    best
}
