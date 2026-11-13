const P: i64 = 1_000_003;

pub fn blocked_walks(rows: i64, cols: i64, blocked: Vec<Vec<i64>>) -> i64 {
    let top = (rows + cols) as usize;
    let mut fact = vec![1i64; top + 1];
    for i in 1..=top {
        fact[i] = fact[i - 1] * i as i64 % P;
    }
    let (mut r, mut b, mut e) = (1i64, fact[top], P - 2);
    while e > 0 {
        if e % 2 == 1 {
            r = r * b % P;
        }
        b = b * b % P;
        e /= 2;
    }
    let mut inv = vec![1i64; top + 1];
    inv[top] = r;
    for i in (1..=top).rev() {
        inv[i - 1] = inv[i] * i as i64 % P;
    }
    let paths = |dr: i64, dc: i64| fact[(dr + dc) as usize] * inv[dr as usize] % P * inv[dc as usize] % P;
    let mut points: Vec<(i64, i64)> = blocked.iter().map(|x| (x[0], x[1])).collect();
    points.sort_unstable();
    points.push((rows, cols));
    let mut good: Vec<i64> = Vec::with_capacity(points.len());
    for i in 0..points.len() {
        let (pr, pc) = points[i];
        let mut ways = paths(pr, pc);
        for j in 0..i {
            let (qr, qc) = points[j];
            if qr <= pr && qc <= pc {
                ways -= good[j] * paths(pr - qr, pc - qc) % P;
            }
        }
        good.push(ways.rem_euclid(P));
    }
    good[good.len() - 1]
}
