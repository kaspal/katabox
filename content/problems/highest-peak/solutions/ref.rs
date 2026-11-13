pub fn peak_index(heights: Vec<i64>) -> i64 {
    let (mut lo, mut hi) = (0, heights.len() - 1);
    while lo < hi {
        let m = (lo + hi) / 2;
        if heights[m] < heights[m + 1] {
            lo = m + 1;
        } else {
            hi = m;
        }
    }
    lo as i64
}
