pub fn distinct_pairs(prices: Vec<i64>, budget: i64) -> i64 {
    let (mut i, mut j) = (0usize, prices.len().saturating_sub(1));
    let mut count = 0;
    while i < j {
        let s = prices[i] + prices[j];
        if s < budget {
            i += 1;
        } else if s > budget {
            j -= 1;
        } else {
            count += 1;
            let (left, right) = (prices[i], prices[j]);
            while i < j && prices[i] == left {
                i += 1;
            }
            while i < j && prices[j] == right {
                j -= 1;
            }
        }
    }
    count
}
