pub fn free_numbers(shelf: Vec<i64>, queries: Vec<i64>) -> Vec<i64> {
    let n = shelf.len();
    let (mut lo, mut hi) = (0usize, n - 1);
    while lo < hi {
        let mid = (lo + hi) / 2;
        if shelf[mid] > shelf[hi] {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    let p = lo;
    let free_below = |i: usize| shelf[(p + i) % n] - (i as i64 + 1);
    queries
        .iter()
        .map(|&k| {
            let (mut a, mut b) = (0usize, n);
            while a < b {
                let mid = (a + b) / 2;
                if free_below(mid) >= k {
                    b = mid;
                } else {
                    a = mid + 1;
                }
            }
            k + a as i64
        })
        .collect()
}
