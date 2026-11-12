pub fn fewest_coins(coins: Vec<i64>, amount: i64) -> i64 {
    // Correct but exponential: try every count of every token.
    fn search(coins: &[i64], i: usize, rest: i64) -> Option<i64> {
        if rest == 0 {
            return Some(0);
        }
        if i == coins.len() {
            return None;
        }
        let mut best: Option<i64> = None;
        for count in 0..=rest / coins[i] {
            if let Some(sub) = search(coins, i + 1, rest - count * coins[i]) {
                if best.is_none_or(|b| count + sub < b) {
                    best = Some(count + sub);
                }
            }
        }
        best
    }
    search(&coins, 0, amount).unwrap_or(-1)
}
