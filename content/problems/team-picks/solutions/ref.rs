const P: i64 = 1_000_003;

fn power(mut b: i64, mut e: i64) -> i64 {
    let mut r = 1;
    b %= P;
    while e > 0 {
        if e % 2 == 1 {
            r = r * b % P;
        }
        b = b * b % P;
        e /= 2;
    }
    r
}

pub fn team_picks(queries: Vec<Vec<i64>>) -> Vec<i64> {
    let top = queries.iter().map(|q| q[0]).max().unwrap() as usize;
    let mut fact = vec![1i64; top + 1];
    for i in 1..=top {
        fact[i] = fact[i - 1] * i as i64 % P;
    }
    let mut inv = vec![1i64; top + 1];
    inv[top] = power(fact[top], P - 2);
    for i in (1..=top).rev() {
        inv[i - 1] = inv[i] * i as i64 % P;
    }
    queries
        .iter()
        .map(|q| {
            let (n, k) = (q[0] as usize, q[1] as usize);
            if k > n {
                0
            } else {
                fact[n] * inv[k] % P * inv[n - k] % P
            }
        })
        .collect()
}
