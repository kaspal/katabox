pub fn min_speed(pages: Vec<i64>, hours: i64) -> i64 {
    let (mut lo, mut hi) = (1i64, *pages.iter().max().unwrap());
    while lo < hi {
        let mid = (lo + hi) / 2;
        let need: i64 = pages.iter().map(|p| (p + mid - 1) / mid).sum();
        if need <= hours {
            hi = mid;
        } else {
            lo = mid + 1;
        }
    }
    lo
}
