pub fn longest_pieces(ribbons: Vec<i64>, orders: Vec<i64>) -> Vec<i64> {
    let m = *ribbons.iter().max().unwrap() as usize;
    let n = ribbons.len() as i64;
    let mut at_most = vec![0i64; m + 1];
    for &r in &ribbons {
        at_most[r as usize] += 1;
    }
    for v in 1..=m {
        at_most[v] += at_most[v - 1];
    }
    let mut count = vec![0i64; m + 1];
    for len in 1..=m {
        let mut total = 0;
        let mut k = len;
        while k <= m {
            total += n - at_most[k - 1];
            k += len;
        }
        count[len] = total;
    }
    orders
        .iter()
        .map(|&need| {
            let (mut lo, mut hi) = (0usize, m);
            while lo < hi {
                let mid = (lo + hi + 1) / 2;
                if count[mid] >= need {
                    lo = mid;
                } else {
                    hi = mid - 1;
                }
            }
            lo as i64
        })
        .collect()
}
