pub fn find_labels(shelf: Vec<i64>, labels: Vec<i64>) -> Vec<i64> {
    let n = shelf.len();
    let (mut lo, mut hi) = (0usize, n - 1);
    while lo < hi {
        let m = (lo + hi) / 2;
        if shelf[m] > shelf[hi] {
            lo = m + 1;
        } else {
            hi = m;
        }
    }
    let start = lo;
    let at = |k: usize| shelf[(k + start) % n];
    labels
        .iter()
        .map(|&x| {
            let (mut a, mut b) = (0usize, n);
            while a < b {
                let m = (a + b) / 2;
                if at(m) < x {
                    a = m + 1;
                } else {
                    b = m;
                }
            }
            if a < n && at(a) == x {
                ((a + start) % n) as i64
            } else {
                -1
            }
        })
        .collect()
}
