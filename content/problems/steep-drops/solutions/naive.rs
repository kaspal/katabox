pub fn steep_drops(prices: Vec<i64>) -> i64 {
    // Correct but O(n²): check every pair of days.
    let mut count = 0;
    for (i, &x) in prices.iter().enumerate() {
        for &y in &prices[i + 1..] {
            if x > 2 * y {
                count += 1;
            }
        }
    }
    count
}
