pub fn kth_gap(posts: Vec<i64>, k: i64) -> i64 {
    let mut a = posts;
    a.sort_unstable();
    let (mut lo, mut hi) = (0, a[a.len() - 1] - a[0]);
    while lo < hi {
        let mid = (lo + hi) / 2;
        let (mut count, mut i) = (0i64, 0usize);
        for j in 0..a.len() {
            while a[j] - a[i] > mid {
                i += 1;
            }
            count += (j - i) as i64;
        }
        if count >= k {
            hi = mid;
        } else {
            lo = mid + 1;
        }
    }
    lo
}
