pub fn tilings_apart(n: i64) -> i64 {
    const MOD: i64 = 1_000_000_007;
    let n = n as usize;
    let mut a = vec![0i64; n + 1];
    let mut b = vec![0i64; n + 1];
    a[0] = 1;
    for i in 1..=n {
        let mut v = a[i - 1] + b[i - 1];
        if i >= 2 {
            v += a[i - 2] + b[i - 2];
        }
        a[i] = v % MOD;
        if i >= 3 {
            b[i] = a[i - 3];
        }
    }
    (a[n] + b[n]) % MOD
}
