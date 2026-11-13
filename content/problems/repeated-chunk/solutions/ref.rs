use std::collections::HashSet;

// Two primes below 2^26, as in the other references.
const M1: i64 = 50_331_653;
const M2: i64 = 25_165_843;
const B: i64 = 131;

pub fn longest_repeat(text: String) -> i64 {
    let s = text.as_bytes();
    let n = s.len();
    let (mut h1, mut h2) = (vec![0i64; n + 1], vec![0i64; n + 1]);
    let (mut p1, mut p2) = (vec![1i64; n + 1], vec![1i64; n + 1]);
    for i in 0..n {
        let c = s[i] as i64;
        h1[i + 1] = (h1[i] * B + c) % M1;
        h2[i + 1] = (h2[i] * B + c) % M2;
        p1[i + 1] = p1[i] * B % M1;
        p2[i + 1] = p2[i] * B % M2;
    }
    let repeats = |len: usize| -> bool {
        let mut seen = HashSet::with_capacity(n - len + 1);
        for i in 0..=n - len {
            let a = (h1[i + len] - h1[i] * p1[len]).rem_euclid(M1);
            let b = (h2[i + len] - h2[i] * p2[len]).rem_euclid(M2);
            if !seen.insert(a * M2 + b) {
                return true;
            }
        }
        false
    };
    let (mut lo, mut hi) = (0usize, n - 1);
    while lo < hi {
        let mid = (lo + hi + 1) / 2;
        if repeats(mid) {
            lo = mid;
        } else {
            hi = mid - 1;
        }
    }
    lo as i64
}
