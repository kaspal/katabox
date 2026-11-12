pub fn fewest_coins(coins: Vec<i64>, amount: i64) -> i64 {
    let amount = amount as usize;
    let inf = amount + 1;
    let mut best = vec![inf; amount + 1];
    best[0] = 0;
    for x in 1..=amount {
        for &c in &coins {
            let c = c as usize;
            if c <= x && best[x - c] + 1 < best[x] {
                best[x] = best[x - c] + 1;
            }
        }
    }
    if best[amount] <= amount { best[amount] as i64 } else { -1 }
}
