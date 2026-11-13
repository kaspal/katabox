use std::io::{self, Read, Write};

fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

/// a⁻¹ modulo m (gcd(a, m) = 1) by the extended Euclidean algorithm; 0 when m = 1.
fn inverse(a: i64, m: i64) -> i64 {
    let (mut old_r, mut r) = (a, m);
    let (mut old_s, mut s) = (1i64, 0i64);
    while r != 0 {
        let q = old_r / r;
        (old_r, r) = (r, old_r - q * r);
        (old_s, s) = (s, old_s - q * s);
    }
    old_s.rem_euclid(m)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_ascii_whitespace().map(|t| t.parse::<i64>().unwrap());
    let t = tokens.next().unwrap();
    let mut out = io::BufWriter::new(io::stdout().lock());
    for _ in 0..t {
        let n = tokens.next().unwrap();
        let (mut x, mut big) = (0i64, 1i64); // the lines so far all leave at x + big·k
        let mut ok = true;
        for _ in 0..n {
            let r = tokens.next().unwrap();
            let m = tokens.next().unwrap();
            if !ok {
                continue;
            }
            let g = gcd(big, m);
            if (r - x) % g != 0 {
                ok = false;
                continue;
            }
            let step = m / g;
            let k = ((r - x) / g).rem_euclid(step) * inverse((big / g) % step, step) % step;
            x += big * k;
            big *= step;
        }
        if ok {
            writeln!(out, "{x}").unwrap();
        } else {
            writeln!(out, "-1").unwrap();
        }
    }
}
