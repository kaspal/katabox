const P: i64 = 1_000_003;

pub fn team_picks(queries: Vec<Vec<i64>>) -> Vec<i64> {
    // Correct but O(k) per query: multiply the k factors of each answer.
    let mut out = Vec::with_capacity(queries.len());
    for q in &queries {
        let (n, k) = (q[0], q[1]);
        if k > n {
            out.push(0);
            continue;
        }
        let k = k.min(n - k);
        let (mut num, mut den) = (1i64, 1i64);
        for i in 0..k {
            num = num * (n - i) % P;
            den = den * (i + 1) % P;
        }
        let (mut inv, mut b, mut e) = (1i64, den, P - 2);
        while e > 0 {
            if e % 2 == 1 {
                inv = inv * b % P;
            }
            b = b * b % P;
            e /= 2;
        }
        out.push(num * inv % P);
    }
    out
}
