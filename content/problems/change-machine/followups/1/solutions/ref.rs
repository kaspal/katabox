pub fn count_ways(coins: Vec<i64>, amount: i64) -> i64 {
    const MOD: i64 = 1_000_000_007;
    let amount = amount as usize;
    let mut ways = vec![0i64; amount + 1];
    ways[0] = 1;
    for c in coins {
        let c = c as usize;
        for a in c..=amount {
            ways[a] = (ways[a] + ways[a - c]) % MOD;
        }
    }
    ways[amount]
}
