pub fn best_pair(prices: Vec<i64>, budget: i64) -> i64 {
    // Correct but O(n²): must run out of time on the max-size tests.
    let mut best = -1;
    let n = prices.len();
    for i in 0..n {
        for j in i + 1..n {
            let total = prices[i] + prices[j];
            if total <= budget && total > best {
                best = total;
            }
        }
    }
    best
}
