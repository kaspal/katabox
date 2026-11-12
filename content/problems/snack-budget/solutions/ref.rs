pub fn most_snacks(prices: Vec<i64>, budget: i64) -> i64 {
    let mut prices = prices;
    prices.sort_unstable();
    let mut spent = 0;
    let mut count = 0;
    for p in prices {
        if spent + p > budget {
            break;
        }
        spent += p;
        count += 1;
    }
    count
}
