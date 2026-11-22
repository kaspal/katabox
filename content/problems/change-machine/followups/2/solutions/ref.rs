pub fn fewest_tokens(tokens: Vec<i64>, amount: i64) -> i64 {
    let amount = amount as usize;
    let inf = tokens.len() as i64 + 1;
    let mut best = vec![inf; amount + 1];
    best[0] = 0;
    for t in tokens {
        let t = t as usize;
        if t > amount {
            continue;
        }
        for a in (t..=amount).rev() {
            if best[a - t] + 1 < best[a] {
                best[a] = best[a - t] + 1;
            }
        }
    }
    if best[amount] < inf { best[amount] } else { -1 }
}
