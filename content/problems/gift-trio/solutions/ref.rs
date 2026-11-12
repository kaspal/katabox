pub fn closest_trio(prices: Vec<i64>, target: i64) -> i64 {
    let mut p = prices;
    p.sort_unstable();
    let n = p.len();
    let mut best = p[0] + p[1] + p[2];
    let mut best_gap = (best - target).abs();
    for i in 0..n - 2 {
        let (mut lo, mut hi) = (i + 1, n - 1);
        while lo < hi {
            let s = p[i] + p[lo] + p[hi];
            let gap = (s - target).abs();
            if gap < best_gap || (gap == best_gap && s < best) {
                best = s;
                best_gap = gap;
            }
            if s < target {
                lo += 1;
            } else if s > target {
                hi -= 1;
            } else {
                return s;
            }
        }
    }
    best
}
