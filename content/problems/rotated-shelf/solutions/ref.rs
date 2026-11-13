pub fn find_label(shelf: Vec<i64>, label: i64) -> i64 {
    // Half-open range [lo, hi) so that the bounds never go below zero.
    let (mut lo, mut hi) = (0usize, shelf.len());
    while lo < hi {
        let m = (lo + hi) / 2;
        if shelf[m] == label {
            return m as i64;
        }
        if shelf[lo] <= shelf[m] {
            if shelf[lo] <= label && label < shelf[m] {
                hi = m;
            } else {
                lo = m + 1;
            }
        } else if shelf[m] < label && label <= shelf[hi - 1] {
            lo = m + 1;
        } else {
            hi = m;
        }
    }
    -1
}
