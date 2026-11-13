pub fn bit_counts(n: i64) -> Vec<i64> {
    let n = n as usize;
    let mut bits = vec![0i64; n + 1];
    for i in 1..=n {
        bits[i] = bits[i >> 1] + (i & 1) as i64;
    }
    bits
}
