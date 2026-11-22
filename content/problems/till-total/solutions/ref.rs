pub fn till_total(prices: Vec<String>) -> i64 {
    prices
        .iter()
        .map(|p| p.replace('.', "").parse::<i64>().unwrap())
        .sum()
}
