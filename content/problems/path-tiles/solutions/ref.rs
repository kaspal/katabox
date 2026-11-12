pub fn tilings(n: i64) -> i64 {
    const MOD: i64 = 1_000_000_007;
    let (mut a, mut b, mut c) = (0i64, 0i64, 1i64);
    for _ in 0..n {
        let next = (a + b + c) % MOD;
        a = b;
        b = c;
        c = next;
    }
    c
}
