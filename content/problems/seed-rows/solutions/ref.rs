pub fn plantings(length: i64, widths: Vec<i64>) -> i64 {
    const MOD: i64 = 1_000_000_007;
    let (n, k) = (length as usize, widths.len());
    let mut ways = vec![0i64; (n + 1) * k];
    let mut total = vec![0i64; n + 1];
    total[0] = 1;
    for x in 1..=n {
        let mut sum = 0;
        for (kind, &w) in widths.iter().enumerate() {
            let w = w as usize;
            if w <= x {
                let v = (total[x - w] - ways[(x - w) * k + kind] + MOD) % MOD;
                ways[x * k + kind] = v;
                sum += v;
            }
        }
        total[x] = sum % MOD;
    }
    total[n]
}
